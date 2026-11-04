package musepluginpolicy

import (
	"net/http"
	"net/http/httptest"
	"strings"
	"testing"

	"github.com/gin-gonic/gin"
)

func TestLocalSessionAndRevocation(t *testing.T) {
	gin.SetMode(gin.TestMode)
	Token = strings.Repeat("a", 64)
	Enabled.Store(true)
	t.Cleanup(func() { Token = ""; Enabled.Store(false) })
	app := gin.New()
	app.Use(Session)
	app.GET("/document", func(c *gin.Context) { c.String(200, "private document") })
	exited := false
	app.POST("/api/system/exit", func(c *gin.Context) { exited = true; c.Status(200) })
	request := func(header, cookie string) *httptest.ResponseRecorder {
		r := httptest.NewRequest("GET", "/document", nil)
		if header != "" {
			r.Header.Set("X-OpenMuse-Local-Session", header)
		}
		if cookie != "" {
			r.Header.Set("Cookie", cookie)
		}
		w := httptest.NewRecorder()
		app.ServeHTTP(w, r)
		return w
	}
	if response := request("", ""); response.Code != http.StatusForbidden {
		t.Fatalf("anonymous local request: %d", response.Code)
	}
	if response := request(strings.Repeat("b", 64), ""); response.Code != http.StatusForbidden {
		t.Fatalf("another plugin's token: %d", response.Code)
	}
	authorized := request(Token, "")
	if authorized.Code != 200 {
		t.Fatalf("authorized initial document: %d", authorized.Code)
	}
	exitRequest := httptest.NewRequest("POST", "/api/system/exit", nil)
	exitRequest.Header.Set("X-OpenMuse-Local-Session", Token)
	exitReply := httptest.NewRecorder()
	app.ServeHTTP(exitReply, exitRequest)
	if exited || !strings.Contains(exitReply.Body.String(), "PLUGIN_LIFECYCLE_MANAGED") {
		t.Fatal("upstream process shutdown escaped the embedding lifecycle boundary")
	}
	cookies := authorized.Result().Cookies()
	if len(cookies) != 1 || !cookies[0].HttpOnly || cookies[0].SameSite != http.SameSiteStrictMode {
		t.Fatalf("session cookie policy: %#v", cookies)
	}
	cookie := cookies[0].Name + "=" + cookies[0].Value
	if response := request("", cookie); response.Code != 200 {
		t.Fatalf("same-view subresource: %d", response.Code)
	}
	Enabled.Store(false)
	if response := request(Token, cookie); response.Code != http.StatusServiceUnavailable {
		t.Fatalf("deactivation did not revoke existing view: %d", response.Code)
	}
	Enabled.Store(true)
	if response := request("", cookie); response.Code != 200 {
		t.Fatalf("reactivation lost same-view session: %d", response.Code)
	}
}
