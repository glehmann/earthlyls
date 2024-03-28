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

pub fn command_description(cmd: &str) -> Option<&str> {
    match cmd {
        "ADD" => Some(ADD),
        "ARG" => Some(ARG),
        "BUILD" => Some(BUILD),
        "CACHE" => Some(CACHE),
        "CMD" => Some(CMD),
        "COPY" => Some(COPY),
        "DO" => Some(DO),
        "ENTRYPOINT" => Some(ENTRYPOINT),
        "ENV" => Some(ENV),
        "EXPOSE" => Some(EXPOSE),
        "FOR" => Some(FOR),
        "FROM_DOCKERFILE" => Some(FROM_DOCKERFILE),
        "FROM" => Some(FROM),
        "FUNCTION" => Some(FUNCTION),
        "GIT CLONE" => Some(GIT_CLONE),
        "HEALTHCHECK" => Some(HEALTHCHECK),
        "HOST" => Some(HOST),
        "IF" => Some(IF),
        "IMPORT" => Some(IMPORT),
        "LABLE" => Some(LABEL),
        "LET" => Some(LET),
        "LOCALLY" => Some(LOCALLY),
        "PROJECT" => Some(PROJECT),
        "RUN" => Some(RUN),
        "SAVE ARTIFACT" => Some(SAVE_ARTIFACT),
        "SAVE IMAGE" => Some(SAVE_IMAGE),
        "SET" => Some(SET),
        "TRY" => Some(TRY),
        "USER" => Some(USER),
        "VERSION" => Some(VERSION),
        "VOLUME" => Some(VOLUME),
        "WAIT" => Some(WAIT),
        "WITH DOCKER" => Some(WITH_DOCKER),
        "WORKDIR" => Some(WORKDIR),
        _ => None,
    }
}
