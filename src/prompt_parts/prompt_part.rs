use git2::Repository;

pub trait RenderablePromptPart {
    fn render(self, repo: &Repository) -> Option<String>;
}
