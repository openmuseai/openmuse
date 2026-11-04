package musepluginpolicy

import (
	"crypto/subtle"
	"net/http"
	"sync/atomic"

	"github.com/gin-gonic/gin"
)

// Token is assigned before the listener starts and never persisted in notebook
// configuration. Cookie scope is only this plugin's isolated local WebView.
var Token string
var OCRLibrary string
var Enabled atomic.Bool

func Session(c *gin.Context) {
	if !Enabled.Load() {
		c.AbortWithStatus(http.StatusServiceUnavailable)
		return
	}
	if len(Token) != 64 {
		c.AbortWithStatus(http.StatusServiceUnavailable)
		return
	}
	cookieName := "muse_siyuan_" + Token[:12]
	header := c.GetHeader("X-OpenMuse-Local-Session")
	cookie, _ := c.Cookie(cookieName)
	if subtle.ConstantTimeCompare([]byte(header), []byte(Token)) != 1 &&
		subtle.ConstantTimeCompare([]byte(cookie), []byte(Token)) != 1 {
		c.AbortWithStatus(http.StatusForbidden)
		return
	}
	if header != "" {
		c.SetSameSite(http.SameSiteStrictMode)
		c.SetCookie(cookieName, Token, 0, "/", "", false, true)
	}
	if c.Request.URL.Path == "/api/system/exit" {
		c.AbortWithStatusJSON(http.StatusOK, gin.H{"code": -1, "msg": "PLUGIN_LIFECYCLE_MANAGED", "data": nil})
		return
	}
	c.Next()
}
