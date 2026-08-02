package tsborrow

import "testing"

func TestParseSuccessfulReport(t *testing.T) {
	result, err := parseOutput([]byte(`{"status":"ok","files":4,"diagnostics":0,"trackedOwners":3,"trackedBorrows":2}` + "\n"))
	if err != nil {
		t.Fatal(err)
	}
	if result.Status != "ok" || result.Files != 4 || result.TrackedOwners != 3 || result.TrackedBorrows != 2 {
		t.Fatalf("unexpected result: %#v", result)
	}
}

func TestParseDiagnostics(t *testing.T) {
	output := `{"file":"src\\socket.ts","code":"TSB002","line":8,"column":3,"message":"use of disposed value: socket"}` + "\n"
	result, err := parseOutput([]byte(output))
	if err != nil {
		t.Fatal(err)
	}
	if len(result.Diagnostics) != 1 || result.Diagnostics[0].File != `src\socket.ts` {
		t.Fatalf("unexpected diagnostics: %#v", result.Diagnostics)
	}
}

func TestParseRejectsInvalidJSON(t *testing.T) {
	if _, err := parseOutput([]byte("not-json\n")); err == nil {
		t.Fatal("expected invalid JSON to fail")
	}
}
