import java.io.File
import javax.inject.Inject
import org.apache.tools.ant.taskdefs.condition.Os
import org.gradle.api.DefaultTask
import org.gradle.api.GradleException
import org.gradle.api.logging.LogLevel
import org.gradle.api.provider.Property
import org.gradle.api.tasks.Input
import org.gradle.api.tasks.TaskAction
import org.gradle.process.ExecOperations

abstract class BuildTask : DefaultTask() {
  @get:Input
  abstract val rootDirRel: Property<String>

  @get:Input
  abstract val target: Property<String>

  @get:Input
  abstract val release: Property<Boolean>

  @get:Inject
  abstract val execOperations: ExecOperations

  @TaskAction
  fun assemble() {
    val executable = """cargo"""
    try {
      runTauriCli(executable)
    } catch (e: Exception) {
      if (Os.isFamily(Os.FAMILY_WINDOWS)) {
        runTauriCli("$executable.cmd")
      } else {
        throw e
      }
    }
  }

  fun runTauriCli(executable: String) {
    val rootDirRelValue = rootDirRel.get()
    val targetValue = target.get()
    val releaseValue = release.get()

    val args = mutableListOf("tauri", "android", "android-studio-script")

    execOperations.exec {
      workingDir(File(project.projectDir, rootDirRelValue))
      executable(executable)
      args(args)
      if (project.logger.isEnabled(LogLevel.DEBUG)) {
        args("-vv")
      } else if (project.logger.isEnabled(LogLevel.INFO)) {
        args("-v")
      }
      if (releaseValue) {
        args("--release")
      }
      args(listOf("--target", targetValue))
    }.assertNormalExitValue()
  }
}