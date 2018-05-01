use std::fmt;

/// #Request
/// This object represents a Telegram Request.
#[derive(Clone, Debug, Serialize, Deserialize)]
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

impl Request {
    /// returns Request type
    pub fn get_type(&self) -> Result<RequestType, String> {
        if !self.message.is_none() {
            return Ok(RequestType::Message);
        }
        if !self.edited_message.is_none() {
            return Ok(RequestType::EditedMesage);
        }
        if !self.inline_query.is_none() {
            return Ok(RequestType::InlineQuery);
        }
        if !self.chosen_inline_result.is_none() {
            return Ok(RequestType::ChosenInlineResult);
        }
        if !self.callback_query.is_none() {
            return Ok(RequestType::CallbackQuery);
        }
        Err(String::from("Unrecognized request type"))
    }

    /// returns update_id value
    pub fn get_update_id(&self) -> &Option<u64> {
        &self.update_id
    }

    /// returns message
    pub fn get_message(&self) -> &Option<Box<Message>> {
        &self.message
    }

    /// returns edited_message
    pub fn get_edited_message(&self) -> &Option<Box<Message>> {
        &self.edited_message
    }

    /// returns inline_query
    pub fn get_inline_query(&self) -> &Option<InlineQuery> {
        &self.inline_query
    }

    /// returns chosen_inline_result
    pub fn get_chosen_inline_result(&self) -> &Option<ChosenInlineResult> {
        &self.chosen_inline_result
    }

    /// returns callback_query
    pub fn get_callback_query(&self) -> &Option<CallbackQuery> {
        &self.callback_query
    }
}

/// #RequestType
/// This object represents the type of a Telegram Request.
pub enum RequestType {
    /// see Message
    Message,
    /// see Message
    EditedMesage,
    /// see InlineQuery
    InlineQuery,
    /// see ChosenInlineResult
    ChosenInlineResult,
    /// see CallbackQuery
    CallbackQuery,
}

/// #User
/// This object represents a Telegram user or bot.
#[derive(Clone, Debug, Serialize, Deserialize)]
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

impl User {
    /// returns id
    pub fn get_id(&self) -> i64 {
        self.id
    }
}

/// #Chat
/// This object represents a chat.
#[derive(Clone, Debug, Serialize, Deserialize)]
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

impl Chat {
    /// returns id
    pub fn get_id(&self) -> i64 {
        self.id
    }
}

/// #Message
/// This object represents a message.
#[derive(Clone, Debug, Serialize, Deserialize)]
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

impl Message {
    /// returns id
    pub fn get_id(&self) -> u64 {
        self.message_id
    }

    /// returns from
    pub fn get_from(&self) -> &User {
        &self.from
    }

    /// returns chat
    pub fn get_chat(&self) -> &Chat {
        &self.chat
    }

    /// returns text
    pub fn get_text(&self) -> &Option<String> {
        &self.text
    }

    /// returns location
    pub fn get_location(&self) -> &Option<Location> {
        &self.location
    }
}

/// #MessageEntity
/// This object represents one special entity in a text message. For example, hashtags, usernames, URLs, etc.
#[derive(Clone, Debug, Serialize, Deserialize)]
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

/// #PhotoSize
/// This object represents one size of a photo or a file / sticker thumbnail.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PhotoSize {
    file_id: String,
    width: i64,
    height: i64,
    #[serde(default)]
    file_size: Option<i64>,
}

/// #Audio
/// This object represents an audio file to be treated as music by the Telegram clients.
#[derive(Clone, Debug, Serialize, Deserialize)]
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

/// #Document
/// This object represents a general file (as opposed to photos, voice messages and audio files).
#[derive(Clone, Debug, Serialize, Deserialize)]
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

/// #Video
/// This object represents a video file.
#[derive(Clone, Debug, Serialize, Deserialize)]
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

/// #Voice
/// This object represents a voice note.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Voice {
    file_id: String,
    duration: i64,
    #[serde(default)]
    mime_type: Option<String>,
    #[serde(default)]
    file_size: Option<u64>,
}

/// #VideoNote
/// This object represents a video message (available in Telegram apps as of v.4.0).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VideoNote {
    file_id: String,
    length: i64,
    duration: i64,
    #[serde(default)]
    thumb: Option<PhotoSize>,
    #[serde(default)]
    file_size: Option<u64>,
}

/// #Contact
/// This object represents a phone contact.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Contact {
    phone_number: String,
    first_name: String,
    #[serde(default)]
    last_name: Option<String>,
    #[serde(default)]
    user_id: Option<i64>,
}

/// #Location
/// This object represents a point on the map.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Location {
    longitude: f64,
    latitude: f64,
}

impl Location {
    /// returns longitude
    pub fn get_longitude(&self) -> f64 {
        self.longitude
    }

    /// returns latitude
    pub fn get_latitude(&self) -> f64 {
        self.latitude
    }
}

/// #Venue
/// This object represents a venue.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Venue {
    location: Location,
    title: String,
    address: String,
    #[serde(default)]
    foursquare_id: Option<String>,
}

/// #UserProfilePhotos
/// This object represent a user's profile pictures.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserProfilePhotos {
    total_count: u64,
    photos: Vec<PhotoSize>,
}

/// #File
/// This object represents a file ready to be downloaded.
/// The file can be downloaded via the link https://api.telegram.org/file/bot<token>/<file_path>.
/// It is guaranteed that the link will be valid for at least 1 hour.
/// When the link expires, a new one can be requested by calling getFile.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct File {
    file_id: String,
    #[serde(default)]
    file_size: Option<u64>,
    #[serde(default)]
    file_path: Option<String>,
}

/// #ReplyKeyboardMarkup
/// This object represents a custom keyboard with reply options
/// (see Introduction to bots for details and examples).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReplyKeyboardMarkup {
    keyboard: Vec<Vec<KeyboardButton>>,
    #[serde(default)]
    resize_keyboard: Option<bool>,
    #[serde(default)]
    one_time_keyboard: Option<bool>,
    #[serde(default)]
    selective: Option<bool>,
}

/// #KeyboardButton
/// This object represents one button of the reply keyboard.
/// For simple text buttons String can be used instead of this object to specify text of the button.
/// Optional fields are mutually exclusive.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KeyboardButton {
    text: String,
    #[serde(default)]
    request_contact: Option<bool>,
    #[serde(default)]
    request_location: Option<bool>,
}

/// #ReplyKeyboardRemove
/// Upon receiving a message with this object, Telegram clients will remove the current custom keyboard
/// and display the default letter-keyboard.
/// By default, custom keyboards are displayed until a new keyboard is sent by a bot.
/// An exception is made for one-time keyboards that are hidden immediately after the user presses a button
/// (see ReplyKeyboardMarkup).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReplyKeyboardRemove {
    remove_keyboard: bool,
    #[serde(default)]
    selective: Option<bool>,
}

/// #InlineKeyboardMarkup
/// This object represents an inline keyboard that appears right next to the message it belongs to.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InlineKeyboardMarkup {
    inline_keyboard: Vec<Vec<InlineKeyboardButton>>,
}

/// #InlineKeyboardButton
/// This object represents one button of an inline keyboard. You must use exactly one of the optional fields.
#[derive(Clone, Debug, Serialize, Deserialize)]
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

/// #CallbackQuery
/// This object represents an incoming callback query from a callback button in an inline keyboard.
/// If the button that originated the query was attached to a message sent by the bot,
/// the field message will be present.
/// If the button was attached to a message sent via the bot (in inline mode),
/// the field inline_message_id will be present.
/// Exactly one of the fields data or game_short_name will be present.
#[derive(Clone, Debug, Serialize, Deserialize)]
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

/// #ForceReply
/// Upon receiving a message with this object, Telegram clients will display a reply interface
/// to the user (act as if the user has selected the bot‘s message and tapped ’Reply').
/// This can be extremely useful if you want to create user-friendly step-by-step interfaces
/// without having to sacrifice privacy mode.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ForceReply {
    force_reply: bool,
    #[serde(default)]
    selective: Option<bool>,
}

/// #ChatPhoto
/// This object represents a chat photo.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChatPhoto {
    small_file_id: String,
    big_file_id: String,
}

/// #ChatMember
/// This object contains information about one member of a chat.
#[derive(Clone, Debug, Serialize, Deserialize)]
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

/// #ResponseParameters
/// Contains information about why a request was unsuccessful.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ResponseParameters {
    migrate_to_chat_id: i64,
    #[serde(default)]
    retry_after: Option<u64>,
}

/// #InputMedia
/// This object represents the content of a media message to be sent. It should be one of
/// InputMediaPhoto
/// InputMediaVideo
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum InputMedia {
    /// see InputMediaPhoto
    #[serde(rename = "photo")]
    Photo(InputMediaPhoto),
    /// see InputMediaVideo
    #[serde(rename = "video")]
    Video(InputMediaVideo),
}

/// #InputMediaPhoto
/// Represents a photo to be sent.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InputMediaPhoto  {
    #[serde(rename="type")]
    type_: String,
    media: String,
    #[serde(default)]
    caption: Option<String>,
    #[serde(default)]
    parse_mode: Option<ParseMode>,
}

/// #InputMediaVideo
/// Represents a video to be sent.
#[derive(Clone, Debug, Serialize, Deserialize)]
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

/// #InputFile
/// This object represents the contents of a file to be uploaded.
/// Must be posted using multipart/form-data in the usual way that files are uploaded via the browser.
pub enum InputFile {
    /// use FileId when using a file already present on Telegram server
    FileId(String),
    /// use Url when uploading a remote file
    Url(String),
    /// use File when uploading a local file
    File(String),
}

/// #Sticker
/// This object represents a sticker.
#[derive(Clone, Debug, Serialize, Deserialize)]
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

/// #StickerSet
/// This object represents a sticker set.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StickerSet {
    name: String,
    title: String,
    contains_masks: bool,
    stickers: Vec<Sticker>,
}

/// #MaskPosition
/// This object describes the position on faces where a mask should be placed by default.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MaskPosition {
    point: String,
    x_shift: f64,
    y_shift: f64,
    scale: f64,
}

/// #InlineQuery
/// This object represents an incoming inline query. When the user sends an empty query,
/// your bot could return some default or trending results.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InlineQuery {
    id: String,
    from: User,
    location: Option<Location>,
    query: String,
    offset: String,
}

/// #InlineQueryResult
/// This object represents one result of an inline query.
/// Telegram clients currently support results of the following 20 types:
/// InlineQueryResultCachedAudio
/// InlineQueryResultCachedDocument
/// InlineQueryResultCachedGif
/// InlineQueryResultCachedMpeg4Gif
/// InlineQueryResultCachedPhoto
/// InlineQueryResultCachedSticker
/// InlineQueryResultCachedVideo
/// InlineQueryResultCachedVoice
/// InlineQueryResultArticle
/// InlineQueryResultAudio
/// InlineQueryResultContact
/// InlineQueryResultGame
/// InlineQueryResultDocument
/// InlineQueryResultGif
/// InlineQueryResultLocation
/// InlineQueryResultMpeg4Gif
/// InlineQueryResultPhoto
/// InlineQueryResultVenue
/// InlineQueryResultVideo
/// InlineQueryResultVoice
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InlineQueryResult {
    /// see InlineQueryResultCachedAudio
    CachedAudio(InlineQueryResultCachedAudio),
    /// see InlineQueryResultCachedDocument
    CachedDocument(InlineQueryResultCachedDocument),
    /// see InlineQueryResultCachedGif
    CachedGif(InlineQueryResultCachedGif),
    /// see InlineQueryResultCachedMpeg4Gif
    CachedMpeg4Gif(InlineQueryResultCachedMpeg4Gif),
    /// see InlineQueryResultCachedPhoto
    CachedPhoto(InlineQueryResultCachedPhoto),
    /// see InlineQueryResultCachedSticker
    CachedSticker(InlineQueryResultCachedSticker),
    /// see InlineQueryResultCachedVideo
    CachedVideo(InlineQueryResultCachedVideo),
    /// see InlineQueryResultCachedVoice
    CachedVoice(InlineQueryResultCachedVoice),
    /// see InlineQueryResultArticle
    Article(InlineQueryResultArticle),
    /// see InlineQueryResultAudio
    Audio(InlineQueryResultAudio),
    /// see InlineQueryResultContact
    Contact(InlineQueryResultContact),
    /// see InlineQueryResultGame
    Game(InlineQueryResultGame),
    /// see InlineQueryResultDocument
    Document(InlineQueryResultDocument),
    /// see InlineQueryResultGif
    Gif(InlineQueryResultGif),
    /// see InlineQueryResultLocation
    Location(InlineQueryResultLocation),
    /// see InlineQueryResultMpeg4Gif
    Mpeg4Gif(InlineQueryResultMpeg4Gif),
    /// see InlineQueryResultPhoto
    Photo(InlineQueryResultPhoto),
    /// see InlineQueryResultVenue
    Venue(InlineQueryResultVenue),
    /// see InlineQueryResultVideo
    Video(InlineQueryResultVideo),
    /// see InlineQueryResultVoice
    Voice(InlineQueryResultVoice),
}

/// #InlineQueryResultArticle
/// Represents a link to an article or web page.
#[derive(Clone, Debug, Serialize, Deserialize)]
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

/// #InlineQueryResultPhoto
/// Represents a link to a photo.
/// By default, this photo will be sent by the user with optional caption.
/// Alternatively, you can use input_message_content to send a message with the specified content
/// instead of the photo.
#[derive(Clone, Debug, Serialize, Deserialize)]
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

/// #InlineQueryResultGif
/// Represents a link to an animated GIF file.
/// By default, this animated GIF file will be sent by the user with optional caption.
/// Alternatively, you can use input_message_content to send a message with the specified content
/// instead of the animation.
#[derive(Clone, Debug, Serialize, Deserialize)]
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

/// #InlineQueryResultMpeg4Gif
/// Represents a link to a video animation (H.264/MPEG-4 AVC video without sound).
/// By default, this animated MPEG-4 file will be sent by the user with optional caption.
/// Alternatively, you can use input_message_content to send a message with the specified content
/// instead of the animation.
#[derive(Clone, Debug, Serialize, Deserialize)]
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

/// #InlineQueryResultVideo
/// Represents a link to a page containing an embedded video player or a video file.
/// By default, this video file will be sent by the user with an optional caption.
/// Alternatively, you can use input_message_content to send a message with the specified content
/// instead of the video.
#[derive(Clone, Debug, Serialize, Deserialize)]
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

/// #InlineQueryResultAudio
/// Represents a link to an mp3 audio file. By default, this audio file will be sent by the user.
/// Alternatively, you can use input_message_content to send a message with the specified content
/// instead of the audio.
#[derive(Clone, Debug, Serialize, Deserialize)]
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

/// #InlineQueryResultVoice
/// Represents a link to a voice recording in an .ogg container encoded with OPUS.
/// By default, this voice recording will be sent by the user.
/// Alternatively, you can use input_message_content to send a message with the specified content
/// instead of the the voice message.
#[derive(Clone, Debug, Serialize, Deserialize)]
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

/// #InlineQueryResultDocument
/// Represents a link to a file. By default, this file will be sent by the user with an optional caption.
/// Alternatively, you can use input_message_content to send a message with the specified content
/// instead of the file. Currently, only .PDF and .ZIP files can be sent using this method.
#[derive(Clone, Debug, Serialize, Deserialize)]
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

/// #InlineQueryResultLocation
/// Represents a location on a map. By default, the location will be sent by the user.
/// Alternatively, you can use input_message_content to send a message with the specified content
/// instead of the location.
#[derive(Clone, Debug, Serialize, Deserialize)]
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

/// #InlineQueryResultVenue
/// Represents a venue. By default, the venue will be sent by the user.
/// Alternatively, you can use input_message_content to send a message with the specified content
/// instead of the venue.
#[derive(Clone, Debug, Serialize, Deserialize)]
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

/// #InlineQueryResultContact
/// Represents a contact with a phone number. By default, this contact will be sent by the user.
/// Alternatively, you can use input_message_content to send a message with the specified content
/// instead of the contact.
#[derive(Clone, Debug, Serialize, Deserialize)]
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

/// #InlineQueryResultGame
/// Represents a Game.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InlineQueryResultGame {
    #[serde(rename="type")]
    type_: String,
    id: String,
    game_short_name: String,
    #[serde(default)]
    reply_markup: Option<InlineKeyboardMarkup>,
}

/// #InlineQueryResultCachedPhoto
/// Represents a link to a photo stored on the Telegram servers.
/// By default, this photo will be sent by the user with an optional caption.
/// Alternatively, you can use input_message_content to send a message with the specified content
/// instead of the photo.
#[derive(Clone, Debug, Serialize, Deserialize)]
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

/// #nlineQueryResultCachedGif
/// Represents a link to an animated GIF file stored on the Telegram servers.
/// By default, this animated GIF file will be sent by the user with an optional caption.
/// Alternatively, you can use input_message_content to send a message with specified content
/// instead of the animation.
#[derive(Clone, Debug, Serialize, Deserialize)]
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

/// #InlineQueryResultCachedMpeg4Gif
/// Represents a link to a video animation (H.264/MPEG-4 AVC video without sound) stored on the
/// Telegram servers. By default, this animated MPEG-4 file will be sent by the user with an optional caption.
/// Alternatively, you can use input_message_content to send a message with the specified content
/// instead of the animation.
#[derive(Clone, Debug, Serialize, Deserialize)]
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

/// #InlineQueryResultCachedSticker
/// Represents a link to a sticker stored on the Telegram servers.
/// By default, this sticker will be sent by the user.
/// Alternatively, you can use input_message_content to send a message with the specified content
/// instead of the sticker.
#[derive(Clone, Debug, Serialize, Deserialize)]
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

/// #InlineQueryResultCachedDocument
/// Represents a link to a file stored on the Telegram servers.
/// By default, this file will be sent by the user with an optional caption.
/// Alternatively, you can use input_message_content to send a message with the specified content
/// instead of the file.
#[derive(Clone, Debug, Serialize, Deserialize)]
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

/// #InlineQueryResultCachedVideo
/// Represents a link to a video file stored on the Telegram servers.
/// By default, this video file will be sent by the user with an optional caption.
/// Alternatively, you can use input_message_content to send a message with the specified content
/// instead of the video.
#[derive(Clone, Debug, Serialize, Deserialize)]
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

/// #InlineQueryResultCachedVoice
/// Represents a link to a voice message stored on the Telegram servers.
/// By default, this voice message will be sent by the user.
/// Alternatively, you can use input_message_content to send a message with the specified content
/// instead of the voice message.
#[derive(Clone, Debug, Serialize, Deserialize)]
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

/// #InlineQueryResultCachedAudio
/// Represents a link to an mp3 audio file stored on the Telegram servers.
/// By default, this audio file will be sent by the user.
/// Alternatively, you can use input_message_content to send a message with the specified content
/// instead of the audio.
#[derive(Clone, Debug, Serialize, Deserialize)]
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

/// #InputMessageContent
/// This object represents the content of a message to be sent as a result of an inline query.
/// Telegram clients currently support the following 4 types:
/// InputTextMessageContent
/// InputLocationMessageContent
/// InputVenueMessageContent
/// InputContactMessageContent
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InputMessageContent {
    /// see InputTextMessageContent
    Text(InputTextMessageContent),
    /// see InputLocationMessageContent
    Location(InputLocationMessageContent),
    /// see InputVenueMessageContent
    Venue(InputVenueMessageContent),
    /// see InputContactMessageContent
    Contact(InputContactMessageContent),
}

/// #InputTextMessageContent
/// Represents the content of a text message to be sent as the result of an inline query.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InputTextMessageContent {
    message_text: String,
    #[serde(default)]
    parse_mode: Option<String>,
    #[serde(default)]
    disable_web_page_preview: Option<bool>,
}

/// #InputLocationMessageContent
/// Represents the content of a location message to be sent as the result of an inline query.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InputLocationMessageContent {
    latitude: f64,
    longitude: f64,
    #[serde(default)]
    live_period: Option<u64>,
}

/// #InputVenueMessageContent
/// Represents the content of a venue message to be sent as the result of an inline query.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InputVenueMessageContent {
    latitude: f64,
    longitude: f64,
    title: String,
    address: String,
    #[serde(default)]
    foursquare_id: Option<String>,
}

/// #InputContactMessageContent
/// Represents the content of a contact message to be sent as the result of an inline query.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InputContactMessageContent {
    phone_number: String,
    first_name: String,
    #[serde(default)]
    last_name: Option<String>,
}

/// #ChosenInlineResult
/// Represents a result of an inline query that was chosen by the user and sent to their chat partner.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChosenInlineResult {
    result_id: String,
    from: User,
    #[serde(default)]
    location: Option<Location>,
    #[serde(default)]
    inline_message_id: Option<String>,
    query: String,
}

/// #LabeledPrice
/// This object represents a portion of the price for goods or services.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LabeledPrice {
    label: String,
    amount: u64,
}

/// #Invoice
/// This object contains basic information about an invoice.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Invoice {
    title: String,
    description: String,
    start_parameter: String,
    currency: String,
    total_amount: u64,
}

/// #ShippingAddress
/// This object represents a shipping address.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ShippingAddress {
    country_code: String,
    state: String,
    city: String,
    street_line1: String,
    street_line2: String,
    post_code: String,
}

/// #OrderInfo
/// This object represents information about an order.
#[derive(Clone, Debug, Serialize, Deserialize)]
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

/// #ShippingOption
/// This object represents one shipping option.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ShippingOption {
    id: String,
    title: String,
    prices: Vec<LabeledPrice>,
}

/// #SuccessfulPayment
/// This object contains basic information about a successful payment.
#[derive(Clone, Debug, Serialize, Deserialize)]
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

/// #ShippingQuery
/// This object contains information about an incoming shipping query.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ShippingQuery {
    id: String,
    from: User,
    invoice_payload: String,
    shipping_address: ShippingAddress,
}

/// #PreCheckoutQuery
/// This object contains information about an incoming pre-checkout query.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PreCheckoutQuery {
    id: String,
    from: User,
    currency: String,
    total_amount: u64,
    invoice_payload: String,
    shipping_option_id: Option<String>,
    order_info: Option<OrderInfo>,
}

/// #Game
/// This object represents a game. Use BotFather to create and edit games,
/// their short names will act as unique identifiers.
#[derive(Clone, Debug, Serialize, Deserialize)]
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

/// #Animation
/// You can provide an animation for your game so that it looks stylish in chats
/// (check out Lumberjack for an example).
/// This object represents an animation file to be displayed in the message containing a game.
#[derive(Clone, Debug, Serialize, Deserialize)]
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

/// #CallbackGame
/// A placeholder, currently holds no information. Use BotFather to set up your game.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CallbackGame {
}

/// #GameHighScore
/// This object represents one row of the high scores table for a game.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GameHighScore {
    position: u64,
    user: User,
    score: u64,
}

/// #ParseMode
/// This object represents Telegram messages formatting options.
#[derive(Clone, Debug)]
pub enum ParseMode {
    /// Markdown style
    Markdown,
    /// HTML style
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

/// #ChatAction
/// This object represents types of action to broadcast.
/// Choose one, depending on what the user is about to receive:
#[derive(Clone, Debug)]
pub enum ChatAction {
    /// typing for text messages
    Typing,
    /// upload_photo for photos
    UploadPhoto,
    /// record_video for videos
    RecordVideo,
    /// upload_video for videos
    UploadVideo,
    /// record_audio for audio files
    RecordAudio,
    /// upload_audio for audio files
    UploadAudio,
    /// upload_document for general files
    UploadDocument,
    /// find_location for location data
    FindLocation,
    /// record_video_note for video notes
    RecordVideoNote,
    /// upload_video_note for video notes
    UploadVideoNote,
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
            ChatAction::RecordVideoNote => "record_video_note",
            ChatAction::UploadVideoNote => "upload_video_note",
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
                    "record_video_note" => Ok(ChatAction::RecordVideoNote),
                    "upload_video_note" => Ok(ChatAction::UploadVideoNote),
                    _ => Err(E::custom(format!("unknown ChatAction value: {}", value))),
                }
            }
        }

        // Deserialize the enum from a u64.
        deserializer.deserialize_str(Visitor)
    }
}

/// #ReplyMarkup
/// This object represents message markup, like keyboards.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReplyMarkup {
    /// see ForceReply
    ForceReply(ForceReply),
    /// see ReplyKeyboardMarkup
    ReplyKeyboard(ReplyKeyboardMarkup),
    /// see ReplyKeyboardRemove
    ReplyKeyboardRemove(ReplyKeyboardRemove),
    /// see InlineKeyboardMarkup
    InlineKeyboard(InlineKeyboardMarkup),
}

impl ReplyMarkup {
    /// helper to create a ReplyMarkup::ForceReply
    pub fn force_reply(force_reply: bool, selective: Option<bool>) -> ReplyMarkup {
        ReplyMarkup::ForceReply(ForceReply {
            force_reply: force_reply,
            selective: selective,
        })
    }

    /// helper to create a ReplyMarkup::ReplyKeyboard
    pub fn reply_keyboard(keyboard: Vec<Vec<KeyboardButton>>, resize_keyboard: Option<bool>, one_time_keyboard: Option<bool>, selective: Option<bool>) -> ReplyMarkup {
        ReplyMarkup::ReplyKeyboard(ReplyKeyboardMarkup {
            keyboard: keyboard,
            resize_keyboard: resize_keyboard,
            one_time_keyboard: one_time_keyboard,
            selective: selective,
        })
    }

    /// helper to create a ReplyMarkup::ReplyKeyboardRemove
    pub fn reply_keyboard_remove(remove_keyboard: bool, selective: Option<bool>) -> ReplyMarkup {
        ReplyMarkup::ReplyKeyboardRemove(ReplyKeyboardRemove {
            remove_keyboard: remove_keyboard,
            selective: selective,
        })
    }

    /// helper to create a ReplyMarkup::InlineKeyboard
    pub fn inline_keyboard(inline_keyboard: Vec<Vec<InlineKeyboardButton>>) -> ReplyMarkup {
        ReplyMarkup::InlineKeyboard(InlineKeyboardMarkup {
            inline_keyboard: inline_keyboard,
        })
    }
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
}"#,
//single position
r#"{
  "update_id":241066346,
  "message":{
    "message_id":2,
    "from":{
    "is_bot": true,
     "last_name":"Test Lastname",
     "type": "private",
     "id":1111111,
     "first_name":"Test Firstname",
     "username":"Testusername"
    },
    "chat":{
     "last_name":"Test Lastname",
     "type": "private",
     "id":1111111,
     "first_name":"Test Firstname",
     "username":"Testusername"
    },
    "date":1520763935,
    "location":{
      "latitude":45.561323,
      "longitude":12.234840
    }
  }
}"#,
//updated position
r#"{
  "update_id":241066349,
  "edited_message":{
    "message_id":4,
    "from":{
    "is_bot": true,
     "last_name":"Test Lastname",
     "type": "private",
     "id":1111111,
     "first_name":"Test Firstname",
     "username":"Testusername"
    },
    "chat":{
     "last_name":"Test Lastname",
     "type": "private",
     "id":1111111,
     "first_name":"Test Firstname",
     "username":"Testusername"
    },
    "date":1520764899,
    "edit_date":1520764972,
    "location":{
      "latitude":45.558900,
      "longitude":12.233439
    }
  }
}"#];
        for s in messages.iter() {
            serde_json::from_str::<Request>(s).expect(s);
        }
        serde_json::from_str::<ParseMode>("\"Markdown\"").unwrap();
        serde_json::from_str::<ChatAction>("\"typing\"").unwrap();
    }
}
