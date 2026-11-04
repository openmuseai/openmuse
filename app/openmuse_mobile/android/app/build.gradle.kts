plugins {
    id("com.android.application")
    // The Flutter Gradle Plugin must be applied after the Android and Kotlin Gradle plugins.
    id("dev.flutter.flutter-gradle-plugin")
}

dependencies {
    implementation("androidx.webkit:webkit:1.14.0")
}

android {
    namespace = "com.openmuseai.mobile"
    compileSdk = flutter.compileSdkVersion
    ndkVersion = flutter.ndkVersion

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }

    defaultConfig {
        applicationId = "com.openmuseai.mobile"
        manifestPlaceholders["openmuseAppLabel"] = "OpenMuse"
        // You can update the following values to match your application needs.
        // For more information, see: https://flutter.dev/to/review-gradle-config.
        minSdk = flutter.minSdkVersion
        targetSdk = flutter.targetSdkVersion
        versionCode = flutter.versionCode
        versionName = flutter.versionName
        ndk { abiFilters += "arm64-v8a" }
    }

    sourceSets.getByName("main").jniLibs.setSrcDirs(listOf(
        rootProject.file("../../../target/paired-relay/android"),
    ))

    // OpenMuse's paired bridge currently ships only arm64. Some
    // dependencies bundle extra 32-bit/x86 libraries; leaving those in the
    // package advertises ABIs that cannot launch the complete application.
    packaging {
        jniLibs {
            excludes += setOf("**/armeabi-v7a/**", "**/x86_64/**", "**/x86/**")
        }
    }

    val releaseKeystore = providers.environmentVariable("OPENMUSE_ANDROID_KEYSTORE").orNull
    if (releaseKeystore != null) {
        signingConfigs.create("release") {
            storeFile = file(releaseKeystore)
            storePassword = providers.environmentVariable("OPENMUSE_ANDROID_STORE_PASSWORD").orNull
            keyAlias = providers.environmentVariable("OPENMUSE_ANDROID_KEY_ALIAS").orNull
            keyPassword = providers.environmentVariable("OPENMUSE_ANDROID_KEY_PASSWORD").orNull
        }
    }

    buildTypes {
        debug {
            providers.environmentVariable("OPENMUSE_ANDROID_DEBUG_APPLICATION_ID_SUFFIX")
                .orNull?.let {
                    applicationIdSuffix = it
                    if (it == ".acceptance") manifestPlaceholders["openmuseAppLabel"] = "OpenMuse QA"
                }
        }
        release {
            providers.environmentVariable("OPENMUSE_ANDROID_RELEASE_APPLICATION_ID_SUFFIX")
                .orNull?.let { applicationIdSuffix = it }
            signingConfig = signingConfigs.findByName("release")
            isMinifyEnabled = true
            isShrinkResources = true
        }
    }
}

val verifyOpenMusePairedNative by tasks.registering {
    val library = rootProject.file(
        "../../../target/paired-relay/android/arm64-v8a/libopenmuse_paired_relay.so",
    )
    inputs.file(library)
    doLast {
        check(library.isFile) {
            "Missing Paired native bridge. Run scripts/build_paired_relay_mobile_artifacts.sh"
        }
    }
}

tasks.named("preBuild").configure {
    dependsOn(
        verifyOpenMusePairedNative,
    )
}

kotlin {
    compilerOptions {
        jvmTarget = org.jetbrains.kotlin.gradle.dsl.JvmTarget.JVM_17
    }
}

flutter {
    source = "../.."
}
