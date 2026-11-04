#!/usr/bin/env lucicfg

lucicfg.check_version("1.30.9", "Please update depot_tools")

# Use LUCI Scheduler BBv2 names and add Scheduler realms configs.
lucicfg.enable_experiment("crbug.com/1182002")

lucicfg.config(
    config_dir = "generated",
    tracked_files = [
        "commit-queue.cfg",
        "cr-buildbucket.cfg",
        "project.cfg",
        "luci-logdog.cfg",
        "luci-milo.cfg",
        "luci-scheduler.cfg",
        "realms.cfg",
    ],
    fail_on_warnings = True,
)

luci.project(
    name = "gn",
    buildbucket = "cr-buildbucket.appspot.com",
    logdog = "luci-logdog",
    milo = "luci-milo",
    scheduler = "luci-scheduler",
    swarming = "chromium-swarm.appspot.com",
    acls = [
        acl.entry(
            [
                acl.BUILDBUCKET_READER,
                acl.LOGDOG_READER,
                acl.PROJECT_CONFIGS_READER,
                acl.SCHEDULER_READER,
            ],
            groups = ["all"],
        ),
        acl.entry([acl.SCHEDULER_OWNER], groups = ["project-gn-committers"]),
        acl.entry([acl.LOGDOG_WRITER], groups = ["luci-logdog-chromium-writers"]),
    ],
)

def builder(name, bucket, os, ref, suffix, caches = None, triggered_by = None):
    luci.builder(
        name = name + suffix,
        bucket = bucket,
        executable = luci.recipe(
            name = "gn" + suffix,
            recipe = "gn",
            cipd_package = "infra/recipe_bundles/gn.googlesource.com/gn",
            cipd_version = ref,
        ),
        caches = caches,
        service_account = "gn-%s-builder@chops-service-accounts.iam.gserviceaccount.com" % bucket,
        execution_timeout = 1 * time.hour,
        dimensions = {"cpu": "x86-64", "os": os, "pool": "luci.flex.%s" % bucket},
        triggered_by = triggered_by,
    )

luci.logdog(
    gs_bucket = "chromium-luci-logdog",
)

luci.milo(
    logo = "https://storage.googleapis.com/chrome-infra-public/logo/gn-logo.png",
)

luci.bucket(name = "ci", acls = [
    acl.entry(
        [acl.BUILDBUCKET_TRIGGERER],
    ),
])

# Shadow bucket for led.
luci.bucket(
    name = "ci.shadow",
    shadows = "ci",
    constraints = luci.bucket_constraints(
        pools = ["luci.flex.ci"],
        service_accounts = [
            "gn-ci-builder@chops-service-accounts.iam.gserviceaccount.com",
        ],
    ),
    bindings = [
        # for led permissions.
        luci.binding(
            roles = "role/buildbucket.creator",
            groups = [
                "project-gn-committers",
                "mdb/chrome-build-access-sphinx",
            ],
        ),
    ],
    dynamic = True,
)

def ci_builder(name, os, ref, suffix, caches = None):
    builder(name, "ci", os, ref = ref, suffix = suffix, caches = caches, triggered_by = ["gn-trigger" + suffix])
    luci.console_view_entry(
        console_view = "gn" + suffix,
        builder = "ci/" + name + suffix,
        short_name = name,
    )

luci.cq(
    submit_max_burst = 4,
    submit_burst_delay = 8 * time.minute,
    gerrit_listener_type = cq.GERRIT_LISTENER_TYPE_LEGACY_POLLER,
)

luci.bucket(name = "try", acls = [
    acl.entry(
        [acl.BUILDBUCKET_TRIGGERER],
        groups = ["project-gn-tryjob-access", "service-account-cq"],
    ),
])

# Shadow bucket for led.
luci.bucket(
    name = "try.shadow",
    shadows = "try",
    constraints = luci.bucket_constraints(
        pools = ["luci.flex.try"],
        service_accounts = [
            "gn-try-builder@chops-service-accounts.iam.gserviceaccount.com",
        ],
    ),
    bindings = [
        # for led permissions.
        luci.binding(
            roles = "role/buildbucket.creator",
            groups = [
                "project-gn-committers",
                "mdb/chrome-build-access-sphinx",
            ],
        ),
    ],
    dynamic = True,
)

luci.binding(
    realm = "try",
    roles = "role/swarming.taskTriggerer",
    groups = "flex-try-led-users",
)

def try_builder(name, os, ref, suffix, caches = None):
    builder(name, "try", os, ref = ref, suffix = suffix, caches = caches)
    luci.cq_tryjob_verifier(
        builder = "try/" + name + suffix,
        cq_group = "gn" + suffix,
    )

def setup_branch(ref, suffix):
    # CI builders should only be enabled for the main branch, since they push
    # to CIPD.
    enable_ci = suffix == ""

    if enable_ci:
        luci.gitiles_poller(
            name = "gn-trigger" + suffix,
            bucket = "ci",
            repo = "https://gn.googlesource.com/gn",
            refs = [ref],
        )

        luci.console_view(
            name = "gn" + suffix,
            title = "gn" + suffix,
            repo = "https://gn.googlesource.com/gn",
            refs = [ref],
            favicon = "https://storage.googleapis.com/chrome-infra-public/logo/favicon.ico",
        )

    luci.cq_group(
        name = "gn" + suffix,
        watch = cq.refset(
            repo = "https://gn.googlesource.com/gn",
            refs = [ref],
        ),
        acls = [
            acl.entry([acl.CQ_COMMITTER], groups = ["project-gn-committers"]),
            acl.entry([acl.CQ_DRY_RUNNER], groups = ["project-gn-tryjob-access"]),
        ],
        retry_config = cq.retry_config(
            single_quota = 1,
            global_quota = 2,
            failure_weight = 1,
            transient_failure_weight = 1,
            timeout_weight = 2,
        ),
    )

    # macOS version for this builder should be synced with
    # https://source.corp.google.com/h/chromium/infra/infra_superproject/+/main:infra_internal/infra/config/subprojects/gn.star
    for name, os, caches in [
        ("linux", "Ubuntu-24.04", None),
        ("mac", "Mac-13", [swarming.cache("macos_sdk")]),
        ("win", "Windows-10", [swarming.cache("windows_sdk")]),
    ]:
        if enable_ci:
            ci_builder(
                name = name,
                os = os,
                ref = ref,
                suffix = suffix,
                caches = caches,
            )

        try_builder(
            name = name,
            os = os,
            ref = ref,
            suffix = suffix,
            caches = caches,
        )

setup_branch(
    ref = "refs/heads/main",
    suffix = "",
)

setup_branch(
    ref = "refs/heads/starlark",
    suffix = "-starlark",
)
