package main

/*
#include <stdlib.h>
*/
import "C"

import (
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"sync"
	"time"
	_ "time/tzdata"
	"unsafe"

	"github.com/siyuan-note/siyuan/kernel/cache"
	"github.com/siyuan-note/siyuan/kernel/conf"
	"github.com/siyuan-note/siyuan/kernel/job"
	"github.com/siyuan-note/siyuan/kernel/model"
	"github.com/siyuan-note/siyuan/kernel/musepluginpolicy"
	"github.com/siyuan-note/siyuan/kernel/server"
	"github.com/siyuan-note/siyuan/kernel/sql"
	"github.com/siyuan-note/siyuan/kernel/util"
)

type options struct {
	Resources  string `json:"resources"`
	Data       string `json:"data"`
	Platform   string `json:"platform"`
	Language   string `json:"language"`
	Session    string `json:"session"`
	OCRLibrary string `json:"ocrLibrary"`
	Timezone   string `json:"timezone"`
}

var lifecycle = struct {
	sync.Mutex
	State  string `json:"state"`
	Origin string `json:"origin,omitempty"`
	Error  string `json:"error,omitempty"`
	Data   string `json:"-"`
}{State: "idle"}

func result(value any) *C.char {
	bytes, _ := json.Marshal(value)
	return C.CString(string(bytes))
}

func status() *C.char {
	lifecycle.Lock()
	defer lifecycle.Unlock()
	return result(map[string]string{"state": lifecycle.State, "origin": lifecycle.Origin, "error": lifecycle.Error})
}

//export muse_siyuan_status
func muse_siyuan_status() *C.char { return status() }

//export muse_siyuan_free
func muse_siyuan_free(value *C.char) { C.free(unsafe.Pointer(value)) }

//export muse_siyuan_start
func muse_siyuan_start(raw *C.char) *C.char {
	var opts options
	if err := json.Unmarshal([]byte(C.GoString(raw)), &opts); err != nil {
		return result(map[string]string{"error": "INVALID_START_OPTIONS"})
	}
	if !filepath.IsAbs(opts.Resources) || !filepath.IsAbs(opts.Data) {
		return result(map[string]string{"error": "ABSOLUTE_PLUGIN_DIRECTORIES_REQUIRED"})
	}
	if len(opts.Session) != 64 {
		return result(map[string]string{"error": "LOCAL_SESSION_REQUIRED"})
	}
	if opts.Platform != "android" && opts.Platform != "ios" && opts.Platform != "macos" {
		return result(map[string]string{"error": "UNSUPPORTED_PLATFORM"})
	}
	for _, name := range []string{"app/appearance/langs/en.json", "app/stage/build/desktop/index.html", "app/stage/build/mobile/index.html"} {
		if _, err := os.Stat(filepath.Join(opts.Resources, name)); err != nil {
			return result(map[string]string{"error": "INCOMPLETE_PLUGIN_RESOURCES: " + name})
		}
	}
	if err := os.MkdirAll(opts.Data, 0700); err != nil {
		return result(map[string]string{"error": "PLUGIN_DATA_UNAVAILABLE"})
	}
	lifecycle.Lock()
	if lifecycle.State != "idle" {
		same := lifecycle.Data == opts.Data
		lifecycle.Unlock()
		if !same {
			return result(map[string]string{"error": "ANOTHER_WORKSPACE_IS_ACTIVE"})
		}
		return status()
	}
	lifecycle.State, lifecycle.Data = "starting", opts.Data
	musepluginpolicy.Token = opts.Session
	musepluginpolicy.OCRLibrary = opts.OCRLibrary
	musepluginpolicy.Enabled.Store(true)
	lifecycle.Unlock()
	go boot(opts)
	return status()
}

func fail(err any) {
	lifecycle.Lock()
	lifecycle.State, lifecycle.Error = "failed", fmt.Sprint(err)
	lifecycle.Unlock()
}

func boot(opts options) {
	defer func() {
		if err := recover(); err != nil {
			fail(err)
		}
	}()
	zone, err := time.LoadLocation(opts.Timezone)
	if err != nil {
		fail("LOCAL_TIMEZONE_UNAVAILABLE")
		return
	}
	time.Local = zone
	// Keep the real backend platform; process exit is handled by the embedding
	// adapter rather than misidentifying a desktop kernel as Android.
	container := opts.Platform
	if container == "macos" {
		container = util.ContainerStd
	}
	util.Mode = "prod"
	util.BootMobile(container, opts.Resources, opts.Data, opts.Language)
	if _, err := os.Stat(filepath.Join(util.ConfDir, "conf.json")); err == nil {
		util.Lang = "" // Existing plugin preferences take precedence over the Host's first-run hint.
	}
	util.ServerPort = "0"
	model.InitJwtKey()
	model.InitConf()
	model.UpdateSystemConf(func(c *conf.System) { c.NetworkServe = false })
	// OpenMuse owns connected services. The embedded editor starts only local
	// persistence/index/history jobs (see the staged cron boundary in build.py).
	go func() {
		defer func() {
			if err := recover(); err != nil {
				fail(err)
			}
		}()
		server.Serve(false, model.Conf.CookieKey)
	}()
	model.InitAppearance()
	sql.InitDatabase(false)
	sql.InitHistoryDatabase(false)
	sql.InitAssetContentDatabase(false)
	model.ApplySearchRuntime()
	model.CleanupEmptyPluginStorageDirs()
	model.InitBoxes()
	model.LoadFlashcards()
	util.LoadAssetsTexts()
	deadline := time.Now().Add(30 * time.Second)
	for !util.HttpServing {
		if time.Now().After(deadline) {
			fail("LOCAL_KERNEL_START_TIMEOUT")
			return
		}
		time.Sleep(25 * time.Millisecond)
	}
	util.SetBooted()
	util.PushClearAllMsg()
	job.StartCron()
	go model.AutoGenerateFileHistory()
	go cache.LoadAssets()
	// Do not load a second plugin system or automatically send notebook content
	// to an independently persisted embedding provider on editor startup.
	if opts.Platform == "macos" {
		model.WatchAssets()
		model.WatchEmojis()
		model.WatchThemes()
	}
	lifecycle.Lock()
	if lifecycle.State != "failed" {
		lifecycle.State = "ready"
		lifecycle.Origin = "http://127.0.0.1:" + util.ServerPort
	}
	lifecycle.Unlock()
}

//export muse_siyuan_flush
func muse_siyuan_flush() *C.char {
	lifecycle.Lock()
	ready := lifecycle.State == "ready"
	lifecycle.Unlock()
	if !ready {
		return result(map[string]string{"error": "KERNEL_NOT_READY"})
	}
	model.FlushTxQueue()
	sql.FlushQueue()
	model.Conf.Save()
	return result(map[string]bool{"saved": true})
}

//export muse_siyuan_suspend
func muse_siyuan_suspend() *C.char {
	// Keep the native singleton loaded until process exit: Go code and live
	// encrypted key sessions cannot be safely dlclosed. Revocation is immediate
	// at the plugin listener; flushing remains plugin-owned.
	musepluginpolicy.Enabled.Store(false)
	return muse_siyuan_flush()
}

//export muse_siyuan_resume
func muse_siyuan_resume() *C.char {
	musepluginpolicy.Enabled.Store(true)
	return status()
}

func main() {}
