// the .md files are generated from earthly’s earthfile reference
// ```sh
// cd earthly
// mdsplit -o doc -l2 docs/earthfile/earthfile.md
// cp -r doc/Earthfile-reference/*.md ../earthlyls/src/descriptions/
// ```

pub const ADD: &str = include_str!("descriptions/ADD-not-supported.md");
pub const ARG: &str = include_str!("descriptions/ARG.md");
pub const BUILD: &str = include_str!("descriptions/BUILD.md");
pub const CACHE: &str = include_str!("descriptions/CACHE.md");
pub const CMD: &str = include_str!("descriptions/CMD-same-as-Dockerfile-CMD.md");
pub const COPY: &str = include_str!("descriptions/COPY.md");
pub const DO: &str = include_str!("descriptions/DO.md");
pub const ENTRYPOINT: &str =
    include_str!("descriptions/ENTRYPOINT-same-as-Dockerfile-ENTRYPOINT.md");
pub const ENV: &str = include_str!("descriptions/ENV-same-as-Dockerfile-ENV.md");
pub const EXPOSE: &str = include_str!("descriptions/EXPOSE-same-as-Dockerfile-EXPOSE.md");
pub const FOR: &str = include_str!("descriptions/FOR.md");
pub const FROM_DOCKERFILE: &str = include_str!("descriptions/FROM-DOCKERFILE.md");
pub const FROM: &str = include_str!("descriptions/FROM.md");
pub const FUNCTION: &str = include_str!("descriptions/FUNCTION.md");
pub const GIT_CLONE: &str = include_str!("descriptions/GIT-CLONE.md");
pub const HEALTHCHECK: &str =
    include_str!("descriptions/HEALTHCHECK-same-as-Dockerfile-HEALTHCHECK.md");
pub const HOST: &str = include_str!("descriptions/HOST.md");
pub const IF: &str = include_str!("descriptions/IF.md");
pub const IMPORT: &str = include_str!("descriptions/IMPORT.md");
pub const LABEL: &str = include_str!("descriptions/LABEL-same-as-Dockerfile-LABEL.md");
pub const LET: &str = include_str!("descriptions/LET.md");
pub const LOCALLY: &str = include_str!("descriptions/LOCALLY.md");
// pub const ONBUILD: &str = include_str!("descriptions/ONBUILD-not-supported.md");
// pub const PIPELINE: &str = include_str!("descriptions/PIPELINE-deprecated.md");
pub const PROJECT: &str = include_str!("descriptions/PROJECT.md");
pub const RUN: &str = include_str!("descriptions/RUN.md");
pub const SAVE_ARTIFACT: &str = include_str!("descriptions/SAVE-ARTIFACT.md");
pub const SAVE_IMAGE: &str = include_str!("descriptions/SAVE-IMAGE.md");
pub const SET: &str = include_str!("descriptions/SET.md");
// pub const SHELL: &str = include_str!("descriptions/SHELL-not-supported.md");
// pub const STOPSIGNAL: &str = include_str!("descriptions/STOPSIGNAL-not-supported.md");
// pub const TRIGGER: &str = include_str!("descriptions/TRIGGER-deprecated.md");
pub const TRY: &str = include_str!("descriptions/TRY-experimental.md");
pub const USER: &str = include_str!("descriptions/USER-same-as-Dockerfile-USER.md");
pub const VERSION: &str = include_str!("descriptions/VERSION.md");
pub const VOLUME: &str = include_str!("descriptions/VOLUME-same-as-Dockerfile-VOLUME.md");
pub const WAIT: &str = include_str!("descriptions/WAIT.md");
pub const WITH_DOCKER: &str = include_str!("descriptions/WITH-DOCKER.md");
pub const WORKDIR: &str = include_str!("descriptions/WORKDIR-same-as-Dockerfile-WORKDIR.md");

pub fn command_description(cmd: &str) -> Option<&'static str> {
    match cmd {
        "add_command" => Some(ADD),
        "arg_command" => Some(ARG),
        "build_command" => Some(BUILD),
        "cache_command" => Some(CACHE),
        "cmd_command" => Some(CMD),
        "copy_command" => Some(COPY),
        "do_command" => Some(DO),
        "entrypoint_command" => Some(ENTRYPOINT),
        "env_command" => Some(ENV),
        "expose_command" => Some(EXPOSE),
        "for_command" => Some(FOR),
        "from_dockerfile_command" => Some(FROM_DOCKERFILE),
        "from_command" => Some(FROM),
        "function_command" => Some(FUNCTION),
        "git_clone_command" => Some(GIT_CLONE),
        "healthcheck_command" => Some(HEALTHCHECK),
        "host_command" => Some(HOST),
        "if_command" => Some(IF),
        "import_command" => Some(IMPORT),
        "lable_command" => Some(LABEL),
        "let_command" => Some(LET),
        "locally_command" => Some(LOCALLY),
        "project_command" => Some(PROJECT),
        "run_command" => Some(RUN),
        "save_artifact_command" => Some(SAVE_ARTIFACT),
        "save_image_command" => Some(SAVE_IMAGE),
        "set_command" => Some(SET),
        "try_command" => Some(TRY),
        "user_command" => Some(USER),
        "version_command" => Some(VERSION),
        "volume_command" => Some(VOLUME),
        "wait_command" => Some(WAIT),
        "with_docker_command" => Some(WITH_DOCKER),
        "workdir_command" => Some(WORKDIR),
        _ => None,
    }
}
