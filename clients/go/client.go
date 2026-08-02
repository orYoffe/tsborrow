// Package tsborrow runs the tsborrow CLI and decodes its stable JSON-lines output.
// The Rust CLI remains the analysis engine; this package contains no duplicate
// ownership logic.
package tsborrow

import (
	"bufio"
	"bytes"
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"os"
	"os/exec"
)

type Diagnostic struct {
	File    string `json:"file"`
	Code    string `json:"code"`
	Line    int    `json:"line"`
	Column  int    `json:"column"`
	Message string `json:"message"`
}

type Result struct {
	Status         string
	Files          int
	TrackedOwners  int
	TrackedBorrows int
	Diagnostics    []Diagnostic
}

type Client struct {
	Binary string
}

func New() Client {
	binary := os.Getenv("TSBORROW_BINARY")
	if binary == "" {
		binary = "tsborrow"
	}
	return Client{Binary: binary}
}

// Check analyzes path. A ViolationsError means analysis completed and found
// diagnostics; other errors mean the CLI could not run or its output was invalid.
func (client Client) Check(ctx context.Context, path string) (Result, error) {
	if client.Binary == "" {
		return Result{}, errors.New("tsborrow: binary path is empty")
	}
	command := exec.CommandContext(ctx, client.Binary, "check", path, "--format", "json")
	var stdout bytes.Buffer
	var stderr bytes.Buffer
	command.Stdout = &stdout
	command.Stderr = &stderr
	runError := command.Run()

	result, parseError := parseOutput(stdout.Bytes())
	if parseError != nil {
		return Result{}, parseError
	}
	if runError == nil {
		return result, nil
	}
	var exitError *exec.ExitError
	if errors.As(runError, &exitError) && exitError.ExitCode() == 1 {
		return result, ViolationsError{Diagnostics: result.Diagnostics}
	}
	return result, fmt.Errorf("tsborrow: command failed: %w: %s", runError, stderr.String())
}

type ViolationsError struct {
	Diagnostics []Diagnostic
}

func (failure ViolationsError) Error() string {
	return fmt.Sprintf("tsborrow: %d ownership violation(s)", len(failure.Diagnostics))
}

type outputLine struct {
	Diagnostic
	Status         string `json:"status"`
	Files          int    `json:"files"`
	TrackedOwners  int    `json:"trackedOwners"`
	TrackedBorrows int    `json:"trackedBorrows"`
}

func parseOutput(output []byte) (Result, error) {
	result := Result{Diagnostics: make([]Diagnostic, 0)}
	scanner := bufio.NewScanner(bytes.NewReader(output))
	for scanner.Scan() {
		var line outputLine
		if err := json.Unmarshal(scanner.Bytes(), &line); err != nil {
			return Result{}, fmt.Errorf("tsborrow: decode output: %w", err)
		}
		if line.Code != "" {
			result.Diagnostics = append(result.Diagnostics, line.Diagnostic)
		}
		if line.Status != "" {
			result.Status = line.Status
			result.Files = line.Files
			result.TrackedOwners = line.TrackedOwners
			result.TrackedBorrows = line.TrackedBorrows
		}
	}
	if err := scanner.Err(); err != nil {
		return Result{}, fmt.Errorf("tsborrow: read output: %w", err)
	}
	return result, nil
}
