use crate::{
    server::{RetError, repository::nix_git_link_repository::NixGitLinkRepository},
    shared::dto::nix_git_link::NixGitLinkDto,
};
#[derive(Debug, Clone)]
pub struct NixGitLinkService {
    repo: NixGitLinkRepository,
}

impl NixGitLinkService {
    pub fn new(repo: NixGitLinkRepository) -> Self {
        Self { repo }
    }
    pub async fn create_many(&self, nix_git_links: Vec<NixGitLinkDto>) -> Result<u64, RetError> {
        let models = nix_git_links.into_iter().map(|dto| dto.into()).collect();
        self.repo.create_many(models).await
    }
    pub async fn create(&self, nix_git_link: NixGitLinkDto) -> Result<(), RetError> {
        let model = nix_git_link.into();
        self.repo.create(model).await
    }
}
