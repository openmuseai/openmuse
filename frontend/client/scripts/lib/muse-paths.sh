# Canonical repo layout after the frontend / backend / middlewares / vendors split.
# Sourced by muse-macos.sh. Do not execute directly.
#
#   frontend/client     Flutter + rust-lib (was client/)
#   frontend/web        AppFlowy-Web clone
#   frontend/website    AppFlowy-Website clone (optional)
#   backend/AppFlowy-Cloud
#   middlewares/dsh     Muse TS packages (was packages/)
#   vendors/deepseek-harness

muse_frontend_dir() {
  printf '%s\n' "$(muse_root)/frontend"
}

muse_client_dir() {
  printf '%s\n' "$(muse_root)/frontend/client"
}

muse_appflowy_frontend() {
  printf '%s\n' "$(muse_root)/frontend/client/frontend"
}

muse_flutter_dir() {
  printf '%s\n' "$(muse_root)/frontend/client/frontend/appflowy_flutter"
}

muse_frontend_scripts() {
  printf '%s\n' "$(muse_client_dir)/scripts"
}

muse_dist_dir() {
  printf '%s\n' "$(muse_client_dir)/dist"
}

muse_web_dir() {
  printf '%s\n' "$(muse_root)/frontend/web"
}

muse_website_dir() {
  local root
  root="$(muse_root)"
  if [[ -d "${root}/frontend/website" ]]; then
    printf '%s\n' "${root}/frontend/website"
  elif [[ -d "${root}/AppFlowy-Website" ]]; then
    printf '%s\n' "${root}/AppFlowy-Website"
  else
    printf '%s\n' "${root}/frontend/website"
  fi
}

muse_cloud_dir() {
  printf '%s\n' "$(muse_root)/backend/AppFlowy-Cloud"
}

muse_packages_root() {
  printf '%s\n' "$(muse_root)/middlewares/dsh"
}

muse_middleware_scripts() {
  printf '%s\n' "$(muse_root)/middlewares/scripts"
}

muse_dsh_deploy_dir() {
  printf '%s\n' "$(muse_root)/middlewares/dsh/deploy"
}

muse_android_deploy_dir() {
  printf '%s\n' "$(muse_root)/local/frontend/deploy/android"
}

muse_harness_dir() {
  printf '%s\n' "$(muse_root)/vendors/deepseek-harness"
}

muse_dsh_patch() {
  printf '%s\n' "$(muse_packages_root)/plugins/dsh-appflowy/cordis.patch.yml"
}

muse_script_build_packages() {
  printf '%s\n' "$(muse_middleware_scripts)/build-muse-packages.sh"
}

muse_script_build_dsh_image() {
  printf '%s\n' "$(muse_middleware_scripts)/build-dsh-image.sh"
}

muse_script_stage_dsh() {
  printf '%s\n' "$(muse_middleware_scripts)/stage-dsh-runtime.sh"
}

muse_script_run_dsh() {
  printf '%s\n' "$(muse_middleware_scripts)/run-dsh-appflowy.sh"
}

muse_script_build_android() {
  printf '%s\n' "$(muse_frontend_scripts)/build-android-client.sh"
}

muse_script_build_mobile_apk() {
  printf '%s\n' "$(muse_frontend_scripts)/build-mobile-apk.sh"
}

muse_script_build_macos() {
  printf '%s\n' "$(muse_frontend_scripts)/build-macos-appflowy.sh"
}

muse_script_build_ios() {
  printf '%s\n' "$(muse_frontend_scripts)/build-ios-client.sh"
}

muse_script_build_mobile_ios() {
  printf '%s\n' "$(muse_frontend_scripts)/build-mobile-ios.sh"
}
