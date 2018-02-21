
use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    #[serde(default)]
    update_id: Option<u64>,
    #[serde(default)]
    message: Option<Box<Message>>,
    #[serde(default)]
    edited_message: Option<Box<Message>>,
    #[serde(default)]
    inline_query: Option<InlineQuery>,
    #[serde(default)]
    chosen_inline_result: Option<ChosenInlineResult>,
    #[serde(default)]
    callback_query: Option<CallbackQuery>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    id: i64,
    is_bot: bool,
    first_name: String,
    #[serde(default)]
    last_name: Option<String>,
    #[serde(default)]
    username: Option<String>,
    #[serde(default)]
    language_code: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Chat {
    id: i64,
    #[serde(rename="type")]
    type_: String,
    #[serde(default)]
    title: Option<String>,
    #[serde(default)]
    username: Option<String>,
    #[serde(default)]
    first_name: Option<String>,
    #[serde(default)]
    last_name: Option<String>,
    #[serde(default)]
    photo: Option<ChatPhoto>,
    #[serde(default)]
    description: Option<String>,
    #[serde(default)]
    invite_link: Option<String>,
    #[serde(default)]
    pinned_message: Option<Box<Message>>,
    #[serde(default)]
    sticker_set_name: Option<String>,
    #[serde(default)]
    can_set_sticker_set: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    message_id: u64,
    from: User,
    date: u64,
    chat: Chat,
    #[serde(default)]
    forward_from: Option<User>,
    #[serde(default)]
    forward_fromchat: Option<Chat>,
    #[serde(default)]
    forward_from_message_id: Option<u64>,
    #[serde(default)]
    forward_signature: Option<String>,
    #[serde(default)]
    reply_to_message: Option<Box<Message>>,
    #[serde(default)]
    edit_date: Option<u64>,
    #[serde(default)]
    media_group_id: Option<String>,
    #[serde(default)]
    author_signature: Option<String>,
    #[serde(default)]
    text: Option<String>,
    #[serde(default)]
    entities: Option<Vec<MessageEntity>>,
    #[serde(default)]
    caption_entities: Option<MessageEntity>,
    #[serde(default)]
    audio: Option<Audio>,
    #[serde(default)]
    document: Option<Document>,
    #[serde(default)]
    game: Option<Game>,
    #[serde(default)]
    photo: Option<Vec<PhotoSize>>,
    #[serde(default)]
    sticker: Option<Sticker>,
    #[serde(default)]
    video_note: Option<VideoNote>,
    #[serde(default)]
    caption: Option<String>,
    #[serde(default)]
    contact: Option<Contact>,
    #[serde(default)]
    location: Option<Location>,
    #[serde(default)]
    venue: Option<Venue>,
    #[serde(default)]
    new_chat_members: Option<Vec<User>>,
    #[serde(default)]
    left_chat_member: Option<User>,
    #[serde(default)]
    new_chat_title: Option<String>,
    #[serde(default)]
    new_chat_photo: Option<Vec<PhotoSize>>,
    #[serde(default)]
    delete_chat_photo: Option<bool>,
    #[serde(default)]
    group_chat_created: Option<bool>,
    #[serde(default)]
    supergroup_chat_created: Option<bool>,
    #[serde(default)]
    channel_chat_created: Option<bool>,
    #[serde(default)]
    migrate_to_chat_id: Option<i64>,
    #[serde(default)]
    migrate_from_chat_id: Option<i64>,
    #[serde(default)]
    pinned_message: Option<Box<Message>>,
    #[serde(default)]
    invoice: Option<Invoice>,
    #[serde(default)]
    successful_payment: Option<SuccessfulPayment>,
    #[serde(default)]
    connected_website: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageEntity {
    #[serde(rename="type")]
    type_: String,
    offset: u64,
    length: u64,
    #[serde(default)]
    url: Option<String>,
    #[serde(default)]
    user: Option<User>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PhotoSize {
    file_id: String,
    width: i64,
    height: i64,
    #[serde(default)]
    file_size: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Audio {
    file_id: String,
    duration: u64,
    #[serde(default)]
    performer: Option<String>,
    #[serde(default)]
    title: Option<String>,
    #[serde(default)]
    mime_type: Option<String>,
    #[serde(default)]
    file_size: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Document {
    file_id: String,
    #[serde(default)]
    thumb: Option<PhotoSize>,
    #[serde(default)]
    file_name: Option<String>,
    #[serde(default)]
    mime_type: Option<String>,
    #[serde(default)]
    file_size: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Video {
    file_id: String,
    width: i64,
    height: i64,
    duration: i64,
    #[serde(default)]
    thumb: Option<PhotoSize>,
    #[serde(default)]
    mime_type: Option<String>,
    #[serde(default)]
    file_size: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Voice {
    file_id: String,
    duration: i64,
    #[serde(default)]
    mime_type: Option<String>,
    #[serde(default)]
    file_size: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VideoNote {
    file_id: String,
    length: i64,
    duration: i64,
    #[serde(default)]
    thumb: Option<PhotoSize>,
    #[serde(default)]
    file_size: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Contact {
    phone_number: String,
    first_name: String,
    #[serde(default)]
    last_name: Option<String>,
    #[serde(default)]
    user_id: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    longitude: f64,
    latitude: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Venue {
    location: Location,
    title: String,
    address: String,
    #[serde(default)]
    foursquare_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserProfilePhotos {
    total_count: u64,
    photos: Vec<PhotoSize>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct File {
    file_id: String,
    #[serde(default)]
    file_size: Option<u64>,
    #[serde(default)]
    file_path: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReplyKeyboardMarkup {
    keyboard: Vec<Vec<KeyboardButton>>,
    #[serde(default)]
    resize_keyboard: Option<bool>,
    #[serde(default)]
    one_time_keyboard: Option<bool>,
    #[serde(default)]
    selective: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KeyboardButton {
    text: String,
    #[serde(default)]
    request_contact: Option<bool>,
    #[serde(default)]
    request_location: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReplyKeyboardRemove {
    remove_keyboard: bool,
    #[serde(default)]
    selective: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InlineKeyboardMarkup {
    inline_keyboard: Vec<Vec<InlineKeyboardButton>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InlineKeyboardButton {
    text: String,
    #[serde(default)]
    url: Option<String>,
    #[serde(default)]
    callback_data: Option<String>,
    #[serde(default)]
    switch_inline_query: Option<String>,
    #[serde(default)]
    switch_inline_query_current_chat: Option<String>,
    #[serde(default)]
    callback_game: Option<CallbackGame>,
    #[serde(default)]
    pay: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallbackQuery {
    id: String,
    from: User,
    #[serde(default)]
    message: Option<Box<Message>>,
    #[serde(default)]
    inline_message_id: Option<String>,
    #[serde(default)]
    chat_instance: Option<String>,
    #[serde(default)]
    data: Option<String>,
    #[serde(default)]
    game_short_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForceReply {
    force_reply: bool,
    #[serde(default)]
    selective: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatPhoto {
    small_file_id: String,
    big_file_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatMember {
    user: User,
    status: String,
    #[serde(default)]
    until_date: Option<u64>,
    #[serde(default)]
    can_be_edited: Option<bool>,
    #[serde(default)]
    can_change_info: Option<bool>,
    #[serde(default)]
    can_post_messages: Option<bool>,
    #[serde(default)]
    can_edit_messages: Option<bool>,
    #[serde(default)]
    can_delete_messages: Option<bool>,
    #[serde(default)]
    can_invite_users: Option<bool>,
    #[serde(default)]
    can_restrict_members: Option<bool>,
    #[serde(default)]
    can_pin_messages: Option<bool>,
    #[serde(default)]
    can_promote_members: Option<bool>,
    #[serde(default)]
    can_send_messages: Option<bool>,
    #[serde(default)]
    can_send_media_messages: Option<bool>,
    #[serde(default)]
    can_send_other_messages: Option<bool>,
    #[serde(default)]
    can_add_web_page_previews: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseParameters {
    migrate_to_chat_id: i64,
    #[serde(default)]
    retry_after: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum InputMedia {
    photo(InputMediaPhoto),
    video(InputMediaVideo),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InputMediaPhoto  {
    #[serde(rename="type")]
    type_: String,
    media: String,
    #[serde(default)]
    caption: Option<String>,
    #[serde(default)]
    parse_mode: Option<ParseMode>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InputMediaVideo  {
    #[serde(rename="type")]
    type_: String,
    media: String,
    #[serde(default)]
    caption: Option<String>,
    #[serde(default)]
    parse_mode: Option<ParseMode>,
    #[serde(default)]
    width: Option<u64>,
    #[serde(default)]
    height: Option<u64>,
    #[serde(default)]
    duration: Option<u64>,
    #[serde(default)]
    supports_streaming: Option<bool>,
}

//#[derive(Debug, Serialize, Deserialize)]
//#[serde(untagged)]
pub enum InputFile {
    FileId(String),
    Url(String),
    File(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Sticker {
    file_id: String,
    width: u64,
    height: u64,
    #[serde(default)]
    thumb: Option<PhotoSize>,
    #[serde(default)]
    emoji: Option<String>,
    #[serde(default)]
    set_name: Option<String>,
    #[serde(default)]
    mask_position: Option<MaskPosition>,
    #[serde(default)]
    file_size: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StickerSet {
    name: String,
    title: String,
    contains_masks: bool,
    stickers: Vec<Sticker>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaskPosition {
    point: String,
    x_shift: f64,
    y_shift: f64,
    scale: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InlineQuery {
    id: String,
    from: User,
    location: Option<Location>,
    query: String,
    offset: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InlineQueryResult {
    CachedAudio(InlineQueryResultCachedAudio),
    CachedDocument(InlineQueryResultCachedDocument),
    CachedGif(InlineQueryResultCachedGif),
    CachedMpeg4Gif(InlineQueryResultCachedMpeg4Gif),
    CachedPhoto(InlineQueryResultCachedPhoto),
    CachedSticker(InlineQueryResultCachedSticker),
    CachedVideo(InlineQueryResultCachedVideo),
    CachedVoice(InlineQueryResultCachedVoice),
    Article(InlineQueryResultArticle),
    Audio(InlineQueryResultAudio),
    Contact(InlineQueryResultContact),
    Game(InlineQueryResultGame),
    Document(InlineQueryResultDocument),
    Gif(InlineQueryResultGif),
    Location(InlineQueryResultLocation),
    Mpeg4Gif(InlineQueryResultMpeg4Gif),
    Photo(InlineQueryResultPhoto),
    Venue(InlineQueryResultVenue),
    Video(InlineQueryResultVideo),
    Voice(InlineQueryResultVoice),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InlineQueryResultArticle {
    #[serde(rename="type")]
    type_: String,
    id: String,
    title: String,
    input_message_content: InputMessageContent,
    #[serde(default)]
    reply_markup: Option<InlineKeyboardMarkup>,
    #[serde(default)]
    url: Option<String>,
    #[serde(default)]
    hide_url: Option<bool>,
    #[serde(default)]
    description: Option<String>,
    #[serde(default)]
    thumb_url: Option<String>,
    #[serde(default)]
    thumb_width: Option<u64>,
    #[serde(default)]
    thumb_height: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InlineQueryResultPhoto {
    #[serde(rename="type")]
    type_: String,
    id: String,
    photo_url: String,
    thumb_url: String,
    #[serde(default)]
    photo_width: Option<u64>,
    #[serde(default)]
    photo_height: Option<u64>,
    #[serde(default)]
    title: Option<String>,
    #[serde(default)]
    description: Option<String>,
    #[serde(default)]
    caption: Option<String>,
    #[serde(default)]
    parse_mode: Option<String>,
    #[serde(default)]
    reply_markup: Option<InlineKeyboardMarkup>,
    #[serde(default)]
    input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InlineQueryResultGif {
    #[serde(rename="type")]
    type_: String,
    id: String,
    gif_url: String,
    #[serde(default)]
    gif_width: Option<u64>,
    #[serde(default)]
    gif_height: Option<u64>,
    #[serde(default)]
    gif_duration: Option<u64>,
    thumb_url: String,
    #[serde(default)]
    title: Option<String>,
    #[serde(default)]
    caption: Option<String>,
    #[serde(default)]
    parse_mode: Option<String>,
    #[serde(default)]
    reply_markup: Option<InlineKeyboardMarkup>,
    #[serde(default)]
    input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InlineQueryResultMpeg4Gif {
    #[serde(rename="type")]
    type_: String,
    id: String,
    mpeg4_url: String,
    #[serde(default)]
    mpeg4_width: Option<u64>,
    #[serde(default)]
    mpeg4_height: Option<u64>,
    #[serde(default)]
    mpeg4_duration: Option<u64>,
    thumb_url: String,
    #[serde(default)]
    title: Option<String>,
    #[serde(default)]
    caption: Option<String>,
    #[serde(default)]
    parse_mode: Option<String>,
    #[serde(default)]
    reply_markup: Option<InlineKeyboardMarkup>,
    #[serde(default)]
    input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InlineQueryResultVideo {
    #[serde(rename="type")]
    type_: String,
    id: String,
    video_url: String,
    mime_type: String,
    thumb_url: String,
    title: String,
    #[serde(default)]
    caption: Option<String>,
    #[serde(default)]
    parse_mode: Option<String>,
    #[serde(default)]
    video_width: Option<u64>,
    #[serde(default)]
    video_height: Option<u64>,
    #[serde(default)]
    video_duration: Option<u64>,
    #[serde(default)]
    description: Option<String>,
    #[serde(default)]
    reply_markup: Option<InlineKeyboardMarkup>,
    #[serde(default)]
    input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InlineQueryResultAudio {
    #[serde(rename="type")]
    type_: String,
    id: String,
    audio_url: String,
    title: String,
    #[serde(default)]
    caption: Option<String>,
    #[serde(default)]
    parse_mode: Option<String>,
    #[serde(default)]
    performer: Option<String>,
    #[serde(default)]
    audio_duration: Option<u64>,
    #[serde(default)]
    reply_markup: Option<InlineKeyboardMarkup>,
    #[serde(default)]
    input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InlineQueryResultVoice {
    #[serde(rename="type")]
    type_: String,
    id: String,
    voice_url: String,
    title: String,
    #[serde(default)]
    caption: Option<String>,
    #[serde(default)]
    parse_mode: Option<String>,
    #[serde(default)]
    voice_duration: Option<u64>,
    #[serde(default)]
    reply_markup: Option<InlineKeyboardMarkup>,
    #[serde(default)]
    input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InlineQueryResultDocument {
    #[serde(rename="type")]
    type_: String,
    id: String,
    title: String,
    #[serde(default)]
    caption: Option<String>,
    #[serde(default)]
    parse_mode: Option<String>,
    document_url: String,
    mime_type: String,
    #[serde(default)]
    description: Option<String>,
    #[serde(default)]
    reply_markup: Option<InlineKeyboardMarkup>,
    #[serde(default)]
    input_message_content: Option<InputMessageContent>,
    #[serde(default)]
    thumb_url: Option<String>,
    #[serde(default)]
    thumb_width: Option<u64>,
    #[serde(default)]
    thumb_height: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InlineQueryResultLocation {
    #[serde(rename="type")]
    type_: String,
    id: String,
    latitude: f64,
    longitude: f64,
    title: String,
    #[serde(default)]
    live_period: Option<u64>,
    #[serde(default)]
    reply_markup: Option<InlineKeyboardMarkup>,
    #[serde(default)]
    input_message_content: Option<InputMessageContent>,
    #[serde(default)]
    thumb_url: Option<String>,
    #[serde(default)]
    thumb_width: Option<u64>,
    #[serde(default)]
    thumb_height: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InlineQueryResultVenue {
    #[serde(rename="type")]
    type_: String,
    id: String,
    latitude: f64,
    longitude: f64,
    title: String,
    address: String,
    #[serde(default)]
    foursquare_id: Option<String>,
    #[serde(default)]
    reply_markup: Option<InlineKeyboardMarkup>,
    #[serde(default)]
    input_message_content: Option<InputMessageContent>,
    #[serde(default)]
    thumb_url: Option<String>,
    #[serde(default)]
    thumb_width: Option<u64>,
    #[serde(default)]
    thumb_height: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InlineQueryResultContact {
    #[serde(rename="type")]
    type_: String,
    id: String,
    phone_number: String,
    first_name: String,
    #[serde(default)]
    last_name: Option<String>,
    #[serde(default)]
    reply_markup: Option<InlineKeyboardMarkup>,
    #[serde(default)]
    input_message_content: Option<InputMessageContent>,
    #[serde(default)]
    thumb_url: Option<String>,
    #[serde(default)]
    thumb_width: Option<u64>,
    #[serde(default)]
    thumb_height: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InlineQueryResultGame {
    #[serde(rename="type")]
    type_: String,
    id: String,
    game_short_name: String,
    #[serde(default)]
    reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InlineQueryResultCachedPhoto {
    #[serde(rename="type")]
    type_: String,
    id: String,
    photo_file_id: String,
    #[serde(default)]
    title: Option<String>,
    #[serde(default)]
    description: Option<String>,
    #[serde(default)]
    caption: Option<String>,
    #[serde(default)]
    parse_mode: Option<String>,
    #[serde(default)]
    reply_markup: Option<InlineKeyboardMarkup>,
    #[serde(default)]
    input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InlineQueryResultCachedGif {
    #[serde(rename="type")]
    type_: String,
    id: String,
    gif_file_id: String,
    #[serde(default)]
    title: Option<String>,
    #[serde(default)]
    caption: Option<String>,
    #[serde(default)]
    parse_mode: Option<String>,
    #[serde(default)]
    reply_markup: Option<InlineKeyboardMarkup>,
    #[serde(default)]
    input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InlineQueryResultCachedMpeg4Gif {
    #[serde(rename="type")]
    type_: String,
    id: String,
    mpeg4_file_id: String,
    #[serde(default)]
    title: Option<String>,
    #[serde(default)]
    caption: Option<String>,
    #[serde(default)]
    parse_mode: Option<String>,
    #[serde(default)]
    reply_markup: Option<InlineKeyboardMarkup>,
    #[serde(default)]
    input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InlineQueryResultCachedSticker {
    #[serde(rename="type")]
    type_: String,
    id: String,
    sticker_file_id: String,
    #[serde(default)]
    reply_markup: Option<InlineKeyboardMarkup>,
    #[serde(default)]
    input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InlineQueryResultCachedDocument {
    #[serde(rename="type")]
    type_: String,
    id: String,
    title: String,
    document_file_id: String,
    #[serde(default)]
    description: Option<String>,
    #[serde(default)]
    caption: Option<String>,
    #[serde(default)]
    parse_mode: Option<String>,
    #[serde(default)]
    reply_markup: Option<InlineKeyboardMarkup>,
    #[serde(default)]
    input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InlineQueryResultCachedVideo {
    #[serde(rename="type")]
    type_: String,
    id: String,
    video_file_id: String,
    title: String,
    #[serde(default)]
    description: Option<String>,
    #[serde(default)]
    caption: Option<String>,
    #[serde(default)]
    parse_mode: Option<String>,
    #[serde(default)]
    reply_markup: Option<InlineKeyboardMarkup>,
    #[serde(default)]
    input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InlineQueryResultCachedVoice {
    #[serde(rename="type")]
    type_: String,
    id: String,
    voice_file_id: String,
    title: String,
    #[serde(default)]
    caption: Option<String>,
    #[serde(default)]
    parse_mode: Option<String>,
    #[serde(default)]
    reply_markup: Option<InlineKeyboardMarkup>,
    #[serde(default)]
    input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InlineQueryResultCachedAudio {
    #[serde(rename="type")]
    type_: String,
    id: String,
    audio_file_id: String,
    #[serde(default)]
    caption: Option<String>,
    #[serde(default)]
    parse_mode: Option<String>,
    #[serde(default)]
    reply_markup: Option<InlineKeyboardMarkup>,
    #[serde(default)]
    input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InputMessageContent {
    Text(InputTextMessageContent),
    Location(InputLocationMessageContent),
    Venue(InputVenueMessageContent),
    Contact(InputContactMessageContent),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InputTextMessageContent {
    message_text: String,
    #[serde(default)]
    parse_mode: Option<String>,
    #[serde(default)]
    disable_web_page_preview: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InputLocationMessageContent {
    latitude: f64,
    longitude: f64,
    #[serde(default)]
    live_period: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InputVenueMessageContent {
    latitude: f64,
    longitude: f64,
    title: String,
    address: String,
    #[serde(default)]
    foursquare_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InputContactMessageContent {
    phone_number: String,
    first_name: String,
    #[serde(default)]
    last_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChosenInlineResult {
    result_id: String,
    from: User,
    #[serde(default)]
    location: Option<Location>,
    #[serde(default)]
    inline_message_id: Option<String>,
    query: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LabeledPrice {
    label: String,
    amount: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Invoice {
    title: String,
    description: String,
    start_parameter: String,
    currency: String,
    total_amount: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShippingAddress {
    country_code: String,
    state: String,
    city: String,
    street_line1: String,
    street_line2: String,
    post_code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderInfo {
    #[serde(default)]
    name: Option<String>,
    #[serde(default)]
    phone_number: Option<String>,
    #[serde(default)]
    email: Option<String>,
    #[serde(default)]
    shipping_address: Option<ShippingAddress>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShippingOption {
    id: String,
    title: String,
    prices: Vec<LabeledPrice>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SuccessfulPayment {
    currency: String,
    total_amount: u64,
    invoice_payload: String,
    #[serde(default)]
    shipping_option_id: Option<String>,
    #[serde(default)]
    order_info: Option<OrderInfo>,
    telegram_payment_charge_id: String,
    provider_payment_charge_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShippingQuery {
    id: String,
    from: User,
    invoice_payload: String,
    shipping_address: ShippingAddress,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreCheckoutQuery {
    id: String,
    from: User,
    currency: String,
    total_amount: u64,
    invoice_payload: String,
    shipping_option_id: Option<String>,
    order_info: Option<OrderInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Game {
    title: String,
    description: String,
    photo: Vec<PhotoSize>,
    #[serde(default)]
    text: Option<String>,
    #[serde(default)]
    text_entities: Option<Vec<MessageEntity>>,
    #[serde(default)]
    animation: Option<Animation>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Animation {
    file_id: String,
    #[serde(default)]
    thumb: Option<PhotoSize>,
    #[serde(default)]
    file_name: Option<String>,
    #[serde(default)]
    mime_type: Option<String>,
    #[serde(default)]
    file_size: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallbackGame {
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameHighScore {
    position: u64,
    user: User,
    score: u64,
}

#[derive(Debug)]
pub enum ParseMode {
    Markdown,
    HTML,
}

impl ToString for ParseMode {
    fn to_string(&self) -> String {
        match *self {
            ParseMode::Markdown => "Markdown".to_owned(),
            ParseMode::HTML => "HTML".to_owned(),
        }
    }
}

impl ::serde::Serialize for ParseMode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: ::serde::Serializer
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> ::serde::Deserialize<'de> for ParseMode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: ::serde::Deserializer<'de>
    {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ParseMode;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("string")
            }

            fn visit_str<E>(self, value: &str) -> Result<ParseMode, E>
                where E: ::serde::de::Error
            {
                // Rust does not come with a simple way of converting a
                // number to an enum, so use a big `match`.
                match value {
                    "Markdown" => Ok(ParseMode::Markdown),
                    "HTML" => Ok(ParseMode::HTML),
                    _ => Err(E::custom(format!("unknown ParseMode value: {}", value))),
                }
            }
        }

        // Deserialize the enum from a u64.
        deserializer.deserialize_str(Visitor)
    }
}

#[derive(Debug)]
pub enum ChatAction {
    Typing,
    UploadPhoto,
    RecordVideo,
    UploadVideo,
    RecordAudio,
    UploadAudio,
    UploadDocument,
    FindLocation,
}

impl ToString for ChatAction {
    fn to_string(&self) -> String {
        (match *self {
            ChatAction::Typing => "typing",
            ChatAction::UploadPhoto => "upload_photo",
            ChatAction::RecordVideo => "record_video",
            ChatAction::UploadVideo => "upload_video",
            ChatAction::RecordAudio => "record_audio",
            ChatAction::UploadAudio => "upload_audio",
            ChatAction::UploadDocument => "upload_document",
            ChatAction::FindLocation => "find_location",
        }).to_owned()
    }
}

impl ::serde::Serialize for ChatAction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: ::serde::Serializer
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> ::serde::Deserialize<'de> for ChatAction {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: ::serde::Deserializer<'de>
    {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ChatAction;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("string")
            }

            fn visit_str<E>(self, value: &str) -> Result<ChatAction, E>
                where E: ::serde::de::Error
            {
                // Rust does not come with a simple way of converting a
                // number to an enum, so use a big `match`.
                match value {
                    "typing" => Ok(ChatAction::Typing),
                    "upload_photo" => Ok(ChatAction::UploadPhoto),
                    "record_video" => Ok(ChatAction::RecordVideo),
                    "upload_video" => Ok(ChatAction::UploadVideo),
                    "record_audio" => Ok(ChatAction::RecordAudio),
                    "upload_audio" => Ok(ChatAction::UploadAudio),
                    "upload_document" => Ok(ChatAction::UploadDocument),
                    "find_location" => Ok(ChatAction::FindLocation),
                    _ => Err(E::custom(format!("unknown ChatAction value: {}", value))),
                }
            }
        }

        // Deserialize the enum from a u64.
        deserializer.deserialize_str(Visitor)
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReplyMarkup {
    ForceReply(ForceReply),
    ReplyKeyboard(ReplyKeyboardMarkup),
    ReplyKeyboardRemove(ReplyKeyboardRemove),
    InlineKeyboard(InlineKeyboardMarkup),
}

#[cfg(test)]
mod tests {
    extern crate serde_json;

    use super::{Request, ParseMode, ChatAction};

    #[test]
    fn it_works() {
        let messages = [
//Message with text
r#"{
"update_id":10000,
"message":{
  "date":1441645532,
  "chat":{
     "last_name":"Test Lastname",
     "id":1111111,
     "type": "private",
     "first_name":"Test Firstname",
     "username":"Testusername"
  },
  "message_id":1365,
  "from":{
    "is_bot": true,
     "last_name":"Test Lastname",
     "id":1111111,
     "first_name":"Test Firstname",
     "username":"Testusername"
  },
  "text":"/start"
}
}"#,
//Forwarded message
r#"{
"update_id":10000,
"message":{
  "date":1441645532,
  "chat":{
     "last_name":"Test Lastname",
     "id":1111111,
     "type": "private",
     "first_name":"Test Firstname",
     "username":"Testusername"
  },
  "message_id":1365,
  "from":{
    "is_bot": true,
     "last_name":"Test Lastname",
     "id":1111111,
     "first_name":"Test Firstname",
     "username":"Testusername"
  },
  "forward_from": {
    "is_bot": true,
     "last_name":"Forward Lastname",
     "id": 222222,
     "first_name":"Forward Firstname"
  },
  "forward_date":1441645550,
  "text":"/start"
}
}"#,
//Forwarded channel message
r#"{
"update_id":10000,
"message":{
  "date":1441645532,
  "chat":{
     "last_name":"Test Lastname",
     "type": "private",
     "id":1111111,
     "first_name":"Test Firstname",
     "username":"Testusername"
  },
  "message_id":1365,
  "from":{
    "is_bot": true,
     "last_name":"Test Lastname",
     "id":1111111,
     "first_name":"Test Firstname",
     "username":"Testusername"
  },
  "forward_from": {
    "is_bot": true,
     "last_name":"Forward Lastname",
     "id": 222222,
     "first_name":"Forward Firstname"
  },
  "forward_date":1441645550,
  "text":"/start"
}
}"#,
//Message with a reply
r#"{
"update_id":10000,
"message":{
  "date":1441645532,
  "chat":{
     "last_name":"Test Lastname",
     "type": "private",
     "id":1111111,
     "first_name":"Test Firstname",
     "username":"Testusername"
  },
  "message_id":1365,
  "from":{
    "is_bot": true,
     "last_name":"Test Lastname",
     "id":1111111,
     "first_name":"Test Firstname",
     "username":"Testusername"
  },
  "text":"/start",
  "reply_to_message":{
      "date":1441645000,
      "chat":{
          "last_name":"Reply Lastname",
          "type": "private",
          "id":1111112,
          "first_name":"Reply Firstname",
          "username":"Testusername"
      },
    "from":{
        "is_bot": true,
        "last_name":"Reply Lastname",
        "id":1111111,
        "first_name":"Reply Firstname",
        "username":"Testusername"
    },
      "message_id":1334,
      "text":"Original"
  }
}
}"#,
//Edited message
r#"{
"update_id":10000,
"edited_message":{
  "date":1441645532,
  "chat":{
     "last_name":"Test Lastname",
     "type": "private",
     "id":1111111,
     "first_name":"Test Firstname",
     "username":"Testusername"
  },
  "message_id":1365,
  "from":{
    "is_bot": true,
     "last_name":"Test Lastname",
     "id":1111111,
     "first_name":"Test Firstname",
     "username":"Testusername"
  },
  "text":"Edited text",
  "edit_date": 1441646600
}
}"#,
//Message with entities
r#"{
"update_id":10000,
"message":{
  "date":1441645532,
  "chat":{
     "last_name":"Test Lastname",
     "type": "private",
     "id":1111111,
     "first_name":"Test Firstname",
     "username":"Testusername"
  },
  "message_id":1365,
  "from":{
    "is_bot": true,
     "last_name":"Test Lastname",
     "id":1111111,
     "first_name":"Test Firstname",
     "username":"Testusername"
  },
  "text":"Bold and italics",
  "entities": [
      {
          "type": "italic",
          "offset": 9,
          "length": 7
      },
      {
          "type": "bold",
          "offset": 0,
          "length": 4
      }
      ]
}
}"#,
//Message with audio
r#"{
"update_id":10000,
"message":{
  "date":1441645532,
  "chat":{
     "last_name":"Test Lastname",
     "type": "private",
     "id":1111111,
     "first_name":"Test Firstname",
     "username":"Testusername"
  },
  "message_id":1365,
  "from":{
    "is_bot": true,
     "last_name":"Test Lastname",
     "id":1111111,
     "first_name":"Test Firstname",
     "username":"Testusername"
  },
  "audio": {
      "file_id": "AwADBAADbXXXXXXXXXXXGBdhD2l6_XX",
      "duration": 243,
      "mime_type": "audio/mpeg",
      "file_size": 3897500,
      "title": "Test music file"
  }
}
}"#,
//Voice message
r#"{
"update_id":10000,
"message":{
  "date":1441645532,
  "chat":{
     "last_name":"Test Lastname",
     "type": "private",
     "id":1111111,
     "first_name":"Test Firstname",
     "username":"Testusername"
  },
  "message_id":1365,
  "from":{
    "is_bot": true,
     "last_name":"Test Lastname",
     "id":1111111,
     "first_name":"Test Firstname",
     "username":"Testusername"
  },
  "voice": {
      "file_id": "AwADBAADbXXXXXXXXXXXGBdhD2l6_XX",
      "duration": 5,
      "mime_type": "audio/ogg",
      "file_size": 23000
  }
}
}"#,
//Message with a document
r#"{
"update_id":10000,
"message":{
  "date":1441645532,
  "chat":{
     "last_name":"Test Lastname",
     "type": "private",
     "id":1111111,
     "first_name":"Test Firstname",
     "username":"Testusername"
  },
  "message_id":1365,
  "from":{
    "is_bot": true,
     "last_name":"Test Lastname",
     "id":1111111,
     "first_name":"Test Firstname",
     "username":"Testusername"
  },
  "document": {
      "file_id": "AwADBAADbXXXXXXXXXXXGBdhD2l6_XX",
      "file_name": "Testfile.pdf",
      "mime_type": "application/pdf",
      "file_size": 536392
  }
}
}"#,
//Inline query
r#"{
"update_id":10000,
"inline_query":{
  "id": "134567890097",
  "from":{
    "is_bot": true,
     "last_name":"Test Lastname",
     "type": "private",
     "id":1111111,
     "first_name":"Test Firstname",
     "username":"Testusername"
  },
  "query": "inline query",
  "offset": ""
}
}"#,
//Chosen inline query
r#"{
"update_id":10000,
"chosen_inline_result":{
  "result_id": "12",
  "from":{
    "is_bot": true,
     "last_name":"Test Lastname",
     "type": "private",
     "id":1111111,
     "first_name":"Test Firstname",
     "username":"Testusername"
  },
  "query": "inline query",
  "inline_message_id": "1234csdbsk4839"
}
}"#,
//Callback query
r#"{
"update_id":10000,
"callback_query":{
  "id": "4382bfdwdsb323b2d9",
  "from":{
    "is_bot": true,
     "last_name":"Test Lastname",
     "type": "private",
     "id":1111111,
     "first_name":"Test Firstname",
     "username":"Testusername"
  },
  "data": "Data from button callback",
  "inline_message_id": "1234csdbsk4839"
}
}"#];
        for s in messages.iter() {
            serde_json::from_str::<Request>(s).expect(s);
        }
        serde_json::from_str::<ParseMode>("\"Markdown\"").unwrap();
        serde_json::from_str::<ChatAction>("\"typing\"").unwrap();
    }
}
