plugins {}

tasks.register("clean").configure {
    delete("build")
}