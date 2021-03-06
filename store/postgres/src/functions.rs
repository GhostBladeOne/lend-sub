use diesel::sql_types::*;

// Create modules for hosting stored procedures
sql_function! {
    revert_block,
    RevertBlock,
    (block_to_revert_hash: Text, subgraph_id: Text)
}
sql_function! {
    current_setting,
    CurrentSetting,
    (setting_name: Text, missing_ok: Bool)
}
sql_function! {
    set_config,
    SetConfig,
    (setting_name: Text, new_value: Text, is_local: Bool)
}
sql_function! {
    attempt_chain_head_update,
    AttemptChainHeadUpdate,
    (net_name: Varchar, ancestor_count: BigInt) -> Array<Varchar>
}
sql_function! {
    lookup_ancestor_block,
    LookupAncestorBlock,
    (start_block_hash: Varchar, ancestor_count: BigInt) -> Nullable<Jsonb>
}

sql_function! {
    build_attribute_index,
    BuildAttributeIndex,
    (
        subgraph_id: Text,
        index_name: Text,
        index_type: Text,
        index_operator: Text,
        jsonb_index: Bool,
        attribute_name: Text,
        entity_name: Text
    )
}

sql_function! {
    pg_notify,
    PGNotify,
    (channel: Text, msg: Text)
}
