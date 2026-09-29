plugins {
    id("com.android.application")
    // The Flutter Gradle Plugin must be applied after the Android and Kotlin Gradle plugins.
    id("dev.flutter.flutter-gradle-plugin")
}

android {
    namespace = "io.openmuse.openmuse_mobile"
    compileSdk = flutter.compileSdkVersion
    ndkVersion = flutter.ndkVersion

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }

    defaultConfig {
        // TODO: Specify your own unique Application ID (https://developer.android.com/studio/build/application-id.html).
        applicationId = "io.openmuse.openmuse_mobile"
        // You can update the following values to match your application needs.
        // For more information, see: https://flutter.dev/to/review-gradle-config.
        minSdk = flutter.minSdkVersion
        targetSdk = flutter.targetSdkVersion
        versionCode = flutter.versionCode
        versionName = flutter.versionName
        ndk { abiFilters += "arm64-v8a" }
    }

    sourceSets.getByName("main").jniLibs.srcDir(
        rootProject.file("../../../target/office-docx/android"),
    )

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
        release {
            signingConfig = signingConfigs.findByName("release")
            isMinifyEnabled = true
            isShrinkResources = true
        }
    }
}

val verifyOpenMuseDocxNative by tasks.registering {
    val library = rootProject.file(
        "../../../target/office-docx/android/arm64-v8a/libopenmuse_office_docx.so",
    )
    inputs.file(library)
    doLast {
        check(library.isFile) {
            "Missing DOCX native engine. Run scripts/build_office_docx_mobile_artifacts.sh"
        }
    }
}

tasks.named("preBuild").configure { dependsOn(verifyOpenMuseDocxNative) }

kotlin {
    compilerOptions {
        jvmTarget = org.jetbrains.kotlin.gradle.dsl.JvmTarget.JVM_17
    }
}

flutter {
    source = "../.."
}
