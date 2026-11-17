import com.android.build.api.dsl.ApplicationExtension
import com.android.build.api.variant.LibraryAndroidComponentsExtension

allprojects {
    repositories {
        google()
        mavenCentral()
    }
}

val newBuildDir: Directory =
    rootProject.layout.buildDirectory
        .dir("../../build")
        .get()
rootProject.layout.buildDirectory.value(newBuildDir)

subprojects {
    val newSubprojectBuildDir: Directory = newBuildDir.dir(project.name)
    project.layout.buildDirectory.value(newSubprojectBuildDir)
}
subprojects {
    project.evaluationDependsOn(":app")
    plugins.withId("com.android.library") {
        extensions.configure<LibraryAndroidComponentsExtension> {
            // Editor dependencies can retain an older compileSdk than their
            // AndroidX dependencies. Align at AGP's public DSL finalization
            // boundary, preserving each library's minSdk and targetSdk.
            finalizeDsl { library ->
                val hostSdk = requireNotNull(
                    project(":app").extensions.getByType<ApplicationExtension>().compileSdk,
                )
                if ((library.compileSdk ?: 0) < hostSdk) {
                    library.compileSdk = hostSdk
                }
            }
        }
    }
}

tasks.register<Delete>("clean") {
    delete(rootProject.layout.buildDirectory)
}
