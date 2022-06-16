table! {
    tipsets (id) {
        id -> Int4,
        height -> Nullable<Int8>,
        cids -> Nullable<Array<Jsonb>>,
        blocks -> Nullable<Array<Jsonb>>,
    }
}
