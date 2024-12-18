use tower_lsp::{
    lsp_types::{
        CodeActionOptions, CodeActionProviderCapability, ColorProviderOptions, CompletionItem,
        CompletionOptions, CompletionOptionsCompletionItem, ExecuteCommandOptions,
        SelectionRangeOptions, SelectionRangeProviderCapability, SemanticTokensServerCapabilities,
        ServerCapabilities, ServerInfo, TextDocumentSyncCapability, WorkDoneProgressOptions,
    },
    Server,
};

use super::DownServer;

pub struct DownCapabilities {}

pub fn capabilities() -> ServerCapabilities {
    return ServerCapabilities {
        code_lens_provider: Default::default(),
        selection_range_provider: Default::default(),
        workspace_symbol_provider: Default::default(),
        rename_provider: Default::default(),
        completion_provider: Some(CompletionOptions {
            resolve_provider: Some(true),
            all_commit_characters: Some(vec!["@".into(), "[".into(), "(".into(), "#".into()]),
            trigger_characters: Some(vec![".".into(), ":".into()]),
            work_done_progress_options: Default::default(),
            completion_item: Some(CompletionOptionsCompletionItem::default()),
        }),
        linked_editing_range_provider: Default::default(),
        semantic_tokens_provider: Default::default(),
        inline_value_provider: Default::default(),
        diagnostic_provider: Default::default(),
        experimental: Default::default(),
        workspace: Default::default(),
        inlay_hint_provider: Default::default(),
        call_hierarchy_provider: Default::default(),
        moniker_provider: Default::default(),
        document_link_provider: Default::default(),
        color_provider: Default::default(),
        folding_range_provider: Default::default(),
        declaration_provider: Default::default(),
        definition_provider: Default::default(),
        execute_command_provider: Some(ExecuteCommandOptions {
            commands: vec!["rust-analyzer.applySourceChange".into()],
            work_done_progress_options: Default::default(),
        }),
        hover_provider: Default::default(),
        code_action_provider: Default::default(),
        position_encoding: Some("utf-8".into()),
        text_document_sync: Default::default(),
        signature_help_provider: Default::default(),
        type_definition_provider: Default::default(),
        implementation_provider: Default::default(),
        references_provider: Default::default(),
        document_highlight_provider: Default::default(),
        document_symbol_provider: Default::default(),
        document_formatting_provider: Default::default(),
        document_range_formatting_provider: Default::default(),
        document_on_type_formatting_provider: Default::default(),
    };
}
