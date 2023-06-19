use std::num;

use clap::{Parser};
use serde::{Deserialize, Serialize};
use colored::Colorize;
use std::io::Write;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
struct Cli {
    #[arg(long)]
	#[clap(allow_hyphen_values = true)]
    file: String,

    #[arg(long)]
	#[clap(allow_hyphen_values = true)]
    outputfile: Option<String>,

	#[arg(long)]
	dontprint: bool,
}

#[derive(Debug, Deserialize)]
struct ChatItem {
    isLive: Option<bool>,
    videoOffsetTimeMsec: Option<String>,
    replayChatItemAction: ReplayChatItemAction,
}

#[derive(Debug, Deserialize)]
struct ReplayChatItemAction {
    actions: Vec<Action>,
}


#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
enum Action {
    AddChatItemAction {
        addChatItemAction: AddChatItemAction,
        clickTrackingParams: Option<String>,
    },
	AddBannerToLiveChatCommand {
		addBannerToLiveChatCommand: serde_json::Value,
		clickTrackingParams: Option<String>,
	},
	AddLiveChatTickerItemAction {
		addLiveChatTickerItemAction: serde_json::Value,
		clickTrackingParams: Option<String>,
	},
	CloseLiveChatActionPanelAction {
		closeLiveChatActionPanelAction: serde_json::Value,
		clickTrackingParams: Option<String>,
	},
	LiveChatReportModerationStateCommand {
		liveChatReportModerationStateCommand: serde_json::Value,
		clickTrackingParams: Option<String>,
	},
	RemoveBannerForLiveChatCommand {
		removeBannerForLiveChatCommand: serde_json::Value,
		clickTrackingParams: Option<String>,
	},
	RemoveChatItemAction {
		removeChatItemAction: serde_json::Value,
		clickTrackingParams: Option<String>,
	},
	RemoveChatItemByAuthorAction {
		removeChatItemByAuthorAction: serde_json::Value,
		clickTrackingParams: Option<String>,
	},
	ReplaceChatItemAction {
		replaceChatItemAction: serde_json::Value,
		clickTrackingParams: Option<String>,
	},
	ShowLiveChatActionPanelAction {
		showLiveChatActionPanelAction: serde_json::Value,
		clickTrackingParams: Option<String>,
	},
	UpdateLiveChatPollAction {
		updateLiveChatPollAction: UpdateLiveChatPollAction,
		clickTrackingParams: Option<String>,
	},
	ReplaceLiveChatRendererAction{
		replaceLiveChatRendererAction: serde_json::Value,
		clickTrackingParams: Option<String>,
	},
    Unknown(serde_json::Value),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
enum ChatItemType {
	LiveChatMembershipItemRenderer {
		liveChatMembershipItemRenderer: LiveChatMembershipItemRenderer,
	},
	LiveChatPaidMessageRenderer {
		liveChatPaidMessageRenderer: PaidMessage,
	},
	LiveChatPlaceholderItemRenderer {
		liveChatPlaceholderItemRenderer: serde_json::Value,
	},
	LiveChatSponsorshipsGiftPurchaseAnnouncementRenderer {
		liveChatSponsorshipsGiftPurchaseAnnouncementRenderer: GiftPurchase,
	},
	LiveChatSponsorshipsGiftRedemptionAnnouncementRenderer {
		liveChatSponsorshipsGiftRedemptionAnnouncementRenderer: GiftRedemptionAnnouncement,
	},
	LiveChatTextMessageRenderer {
		liveChatTextMessageRenderer: serde_json::Value,
	},
	LiveChatViewerEngagementMessageRenderer {
		liveChatViewerEngagementMessageRenderer: serde_json::Value,
	},
	LiveChatPaidStickerRenderer{
		liveChatPaidStickerRenderer: PaidSticker,
	},
	LiveChatModeChangeMessageRenderer{
		liveChatModeChangeMessageRenderer: serde_json::Value,
	},
	Unknown(serde_json::Value)
}

#[derive(Serialize, Deserialize, Debug)]
struct PaidSticker {
	authorExternalChannelId: String,
	authorName: SimpleText,
	authorNameTextColor: i64,
	authorPhoto: AuthorPhotos,
	backgroundColor: i64,
	contextMenuAccessibility: serde_json::Value,
	contextMenuEndpoint: serde_json::Value,
	id: String,
	moneyChipBackgroundColor: i64,
	moneyChipTextColor: i64,
	purchaseAmountText: SimpleText,
	sticker: PollImage,
	stickerDisplayHeight: i64,
	stickerDisplayWidth: i64,
	timestampText: Option<SimpleText>,
	timestampUsec: String,
	trackingParams: Option<String>,
}


#[derive(Serialize, Deserialize, Debug)]
struct GiftRedemptionAnnouncement {
	authorExternalChannelId: String,
	authorName: SimpleText,
	authorPhoto: AuthorPhotos,
	contextMenuAccessibility: serde_json::Value,
	contextMenuEndpoint: serde_json::Value,
	id: String,
	message: RunsContainer,
	timestampText: Option<SimpleText>,
	timestampUsec: String,
	trackingParams: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct GiftPurchase {
	authorExternalChannelId: String,
	header: GiftPurchaseHeader,
	id: String,
	timestampUsec: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct GiftPurchaseHeader {
	liveChatSponsorshipsHeaderRenderer: GiftPurchaseHeaderRenderer,
}

#[derive(Serialize, Deserialize, Debug)]
struct GiftPurchaseHeaderRenderer {
	authorBadges: Option<Vec<serde_json::Value>>,
	authorName: SimpleText,
	authorPhoto: AuthorPhotos,
	contextMenuAccessibility: serde_json::Value,
	contextMenuEndpoint: serde_json::Value,
	image: serde_json::Value,
	primaryText: RunsContainer,
}


#[derive(Serialize, Deserialize, Debug)]
struct LiveChatMembershipItemRenderer {
	authorBadges: Option<Vec<serde_json::Value>>,
	authorExternalChannelId: String,
	authorName: SimpleText,
	authorPhoto: AuthorPhotos,
	contextMenuAccessibility: serde_json::Value,
	contextMenuEndpoint: serde_json::Value,
	headerPrimaryText: Option<RunsContainer>,
	headerSubtext: HeaderSubtextType,
	id: String,
	message: Option<RunsContainer>,
	timestampText: Option<SimpleText>,
	timestampUsec: String,
	trackingParams: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
enum HeaderSubtextType {
	Runs(RunsContainer),
	SimpleText(SimpleText),

}


#[derive(Serialize, Deserialize, Debug)]
struct PaidMessage {
	authorBadges: Option<Vec<serde_json::Value>>,
	authorExternalChannelId: String,
	authorName: SimpleText,
	authorNameTextColor: i64,
	authorPhoto: AuthorPhotos,
	bodyBackgroundColor: i64,
	bodyTextColor: i64,
	//contextMenuAccessibility: serde_json::Value,
	//contextMenuEndpoint: serde_json::Value,
	headerBackgroundColor: i64,
	headerTextColor: i64,
	id: String,
	message: Option<RunsContainer>,
	purchaseAmountText: SimpleText,
	textInputBackgroundColor: i64,
	timestampText: Option<SimpleText>,
	timestampColor: i64,
	timestampUsec: String,
	//trackingParams: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct AuthorPhotos {
	thumbnails: Vec<Thumbnail>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Thumbnail {
	height: Option<i64>,
	url: String,
	width: Option<i64>
}

#[derive(Serialize, Deserialize, Debug)]
struct AddChatItemAction {
	item: ChatItemType,
	clientId: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct UpdateLiveChatPollAction {
	pollToUpdate: PollToUpdate
}

#[derive(Serialize, Deserialize, Debug)]
struct PollToUpdate {
	pollRenderer: PollRenderer
}

#[derive(Serialize, Deserialize, Debug)]
struct PollRenderer {
	choices:Vec<PollChoice>,
	liveChatPollId:String,
	header:PollHeader,
}

#[derive(Serialize, Deserialize, Debug)]
struct PollHeader {
	pollHeaderRenderer: PollHeaderRenderer
}

#[derive(Serialize, Deserialize, Debug)]
struct PollHeaderRenderer {
	pollQuestion: RunsContainer,
	//thubnail: PollThumbnail,
	metadataText: RunsContainer,
	liveChatPollType: String,
	//contextMenuButton: PollContextMenuButton,
}



#[derive(Serialize, Deserialize, Debug)]
struct RunsContainer {
	runs: Vec<RunsTypes>,
	bold: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
struct PollChoice {
	selected:bool,
	text: RunsContainer,
	votePercentage: SimpleText,
	voteRatio:f32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
enum RunsTypes {
	Text{
		text:String,
		italics:Option<bool>,
		bold:Option<bool>,
	},
	Emoji{
		emoji:PollRunsEmoji
	},
	Unknown(serde_json::Value)
}

#[derive(Serialize, Deserialize, Debug)]
struct PollRunsEmoji {
	emojiId:String,
	image:PollImage,
	searchTerms:Option<Vec<String>>,
	shortcuts:Option<Vec<String>>,
	isCustomEmoji:Option<bool>,
	variantIds:Option<Vec<String>>,
	supportsSkinTone:Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
struct PollImage {
	accessibility: EmoteAccessability,
	thumbnails: Vec<Thumbnail>
}

#[derive(Serialize, Deserialize, Debug)]
struct EmoteAccessability {
	accessibilityData: AccessibilityData
}

#[derive(Serialize, Deserialize, Debug)]
struct AccessibilityData {
	label: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct SimpleText {
	simpleText:String
}

fn main() {
    // get file from cli using clap
    let args = Cli::parse();
    let file = std::fs::read_to_string(&args.file).expect("could not read file");
	
	let mut num_superchats = 0;
	let mut num_messages = 0;
	let mut num_memberships = 0;
	let mut num_gifts = 0;
	let mut num_redemptions = 0;
	let mut num_stickers = 0;

	let mut donations = Vec::new();
	macro_rules! println {
		($($rest:tt)*) => {
			if !args.dontprint {
				std::println!($($rest)*)
			}
		}
	}
	// iterate over every line in the file
	for line in file.lines() {
		// parse the line as chat item
		let chat_item: ChatItem = serde_json::from_str(line).expect("could not parse line");
		for action in chat_item.replayChatItemAction.actions {
			match action {
				Action::AddChatItemAction { addChatItemAction, clickTrackingParams } => {
					// adds things in the chat like messages, donations, join button, etc.
					match addChatItemAction.item {
						ChatItemType::LiveChatPaidMessageRenderer { liveChatPaidMessageRenderer } => {
							// donation
							num_superchats += 1;
							//continue;
							

							// liveChatPaidMessageRenderer.bodyBackgroundColor is the raw decimal value of the color
							// convert it to hex
							let background_color = format!("{:x}", liveChatPaidMessageRenderer.bodyBackgroundColor);
							// background_color is in argb format with 2 digits for each part
							// split into alpha, red, green, blue
							// this should be safe because the alpha is always ff
							
							assert!(background_color.len() == 8, "background color is not 8 characters long: {}", background_color);

							// split into alpha, red, green, blue u8 values
							let alpha = u8::from_str_radix(&background_color[0..2], 16).expect("could not parse alpha");
							let red = u8::from_str_radix(&background_color[2..4], 16).expect("could not parse red");
							let green = u8::from_str_radix(&background_color[4..6], 16).expect("could not parse green");
							let blue = u8::from_str_radix(&background_color[6..8], 16).expect("could not parse blue");
							
							let sep = "==========donation start==========".black().on_truecolor(red, green, blue);
							println!("{}",sep);
							let timestring = match liveChatPaidMessageRenderer.timestampText {
								// should always exist in replays
								Some(timestamp) => timestamp.simpleText,
								// if it doesn't exist fallback to .timestampUsec
								// and convert from microseconds to datetime

								None => {
									use chrono::NaiveDateTime;
									let timestamp = liveChatPaidMessageRenderer.timestampUsec;
									let timestamp = timestamp.parse::<i64>().expect("could not parse timestamp");
									let timestamp = timestamp / 1000000;
									let timestamp = NaiveDateTime::from_timestamp_opt(timestamp, 0).expect("could not convert timestamp to datetime");
									timestamp.format("%Y-%m-%d %H:%M:%S").to_string()
								}
							};

							println!("time: {}", timestring);
							//println!("donation: {:#?}", liveChatPaidMessageRenderer);
							// print username and channel id
							println!("username: {}, channel: https://youtube.com/channel/{}", liveChatPaidMessageRenderer.authorName.simpleText, liveChatPaidMessageRenderer.authorExternalChannelId);
							// print amount
							println!("amount: {}", liveChatPaidMessageRenderer.purchaseAmountText.simpleText);

							// print background color
							//println!("background color: {}", background_color);

							// print message
							let mut message = String::new();
							if let Some(message_run) = liveChatPaidMessageRenderer.message {
								for run in message_run.runs {
									match run {
										RunsTypes::Text { text, italics, bold } => {
											message.push_str(&text);
										},
										RunsTypes::Emoji { emoji } => {
											if let Some(is_custom_emoji) = emoji.isCustomEmoji {
												if is_custom_emoji {
													message.push_str(format!(":{}:",&emoji.image.accessibility.accessibilityData.label).as_str());
												} else {
													message.push_str(&emoji.emojiId);
												}
											} else {
												message.push_str(&emoji.emojiId);
											}
	
											//poll_name.push_str(&emoji.image.accessibility.accessibilityData.label);
										},
										RunsTypes::Unknown(unknown) => {
											println!("UNKNOWN VALUE IN RUNS: {:#?}", unknown);
										},
									}
								}
								println!("message: {}", message);
							}
							
							// struct for exporting to json
							#[derive(Serialize, Deserialize, Debug)]
							struct Donation {
								#[serde(rename = "type")]
								json_type: String,
								username: String,
								channel_id: String,
								amount: String,
								message: Option<String>,
								time: String,
								header_color: i64,
								body_color: i64,
								thumbnail_url: String,
							}

							// if message length is 0, set it to None
							let message = if message.len() == 0 {
								None
							} else {
								Some(message)
							};

							// thumbnail_url is the last url in the thumbnails array
							// create donation struct
							let donation = Donation {
								json_type: "Donation".to_string(),
								thumbnail_url: liveChatPaidMessageRenderer.authorPhoto.thumbnails.last().expect("could not get thumbnail url").url.clone(),
								username: liveChatPaidMessageRenderer.authorName.simpleText.clone(),
								channel_id: liveChatPaidMessageRenderer.authorExternalChannelId.clone(),
								amount: liveChatPaidMessageRenderer.purchaseAmountText.simpleText.clone(),
								message: message,
								time: timestring.clone(),
								header_color: liveChatPaidMessageRenderer.headerBackgroundColor,
								body_color: liveChatPaidMessageRenderer.bodyBackgroundColor,
							};
							donations.push(serde_json::to_string(&donation).expect("could not serialize donation"));

							println!("===========donation end===========");

						},
						ChatItemType::LiveChatMembershipItemRenderer { liveChatMembershipItemRenderer } => {
							num_memberships+=1;
							// join button
							let sep = "=========membership start=========".black().on_truecolor(10, 128, 67);
							println!("{}",sep);
							let timestring = match liveChatMembershipItemRenderer.timestampText {
								// should always exist in replays
								Some(timestamp) => timestamp.simpleText,
								// if it doesn't exist fallback to .timestampUsec
								// and convert from microseconds to datetime

								None => {
									use chrono::NaiveDateTime;
									let timestamp = liveChatMembershipItemRenderer.timestampUsec;
									let timestamp = timestamp.parse::<i64>().expect("could not parse timestamp");
									let timestamp = timestamp / 1000000;
									let timestamp = NaiveDateTime::from_timestamp_opt(timestamp, 0).expect("could not convert timestamp to datetime");
									timestamp.format("%Y-%m-%d %H:%M:%S").to_string()
								}
							};
							println!("time: {}", timestring);
							//println!("membership: {:#?}", liveChatMembershipItemRenderer);
							// print username and channel id
							println!("username: {}, channel: https://youtube.com/channel/{}", liveChatMembershipItemRenderer.authorName.simpleText, liveChatMembershipItemRenderer.authorExternalChannelId);
							// print number of months
							
							// if the user just joined the channel then headerPrimaryText will be None
							// and the welcome message will be in the headerSubtext field
							// so check if headerPrimaryText is None and if it is then say 0 months
							// otherwise the number of months is in headerPrimaryText

							let mut months = String::new();
							if let Some(headerPrimaryText) = liveChatMembershipItemRenderer.headerPrimaryText {
								let mut message = String::new();
								for runs in headerPrimaryText.runs {
									match runs {
										RunsTypes::Text { text, italics, bold } => {
											message.push_str(&text);
										},
										RunsTypes::Emoji { emoji } => {
											if let Some(is_custom_emoji) = emoji.isCustomEmoji {
												if is_custom_emoji {
													message.push_str(format!(":{}:",&emoji.image.accessibility.accessibilityData.label).as_str());
												} else {
													message.push_str(&emoji.emojiId);
												}
											} else {
												message.push_str(&emoji.emojiId);
											}
	
											//poll_name.push_str(&emoji.image.accessibility.accessibilityData.label);
										},
										RunsTypes::Unknown(unknown) => {
											println!("UNKNOWN VALUE IN RUNS: {:#?}", unknown);
										},
									}
								}
								println!("months: {}", message);
								months = message;
							} else {
								println!("months: New member.");
								months = "New member.".to_string();
							}

							// print message if there is one
							let mut message = String::new();
							if let Some(lmessage) = liveChatMembershipItemRenderer.message {
								for run in lmessage.runs {
									match run {
										RunsTypes::Text { text, italics, bold } => {
											message.push_str(&text);
										},
										RunsTypes::Emoji { emoji } => {
											if let Some(is_custom_emoji) = emoji.isCustomEmoji {
												if is_custom_emoji {
													message.push_str(format!(":{}:",&emoji.image.accessibility.accessibilityData.label).as_str());
												} else {
													message.push_str(&emoji.emojiId);
												}
											} else {
												message.push_str(&emoji.emojiId);
											}
	
											//poll_name.push_str(&emoji.image.accessibility.accessibilityData.label);
										},
										RunsTypes::Unknown(unknown) => {
											println!("UNKNOWN VALUE IN RUNS: {:#?}", unknown);
										},
									}
								}
							}

							// struct for exporting to json
							#[derive(Serialize, Deserialize, Debug)]
							struct Membership {
								#[serde(rename = "type")]
								json_type: String,
								username: String,
								channel_id: String,
								months: String,
								message: Option<String>,
								time: String,
								header_color: i64,
								body_color: i64,
								thumbnail_url: String,
							}

							// if message length is 0, set it to None
							let message = if message.len() == 0 {
								None
							} else {
								Some(message)
							};

							// thumbnail_url is the last url in the thumbnails array
							// create donation struct
							let membership = Membership {
								json_type: "Membership".to_string(),
								thumbnail_url: liveChatMembershipItemRenderer.authorPhoto.thumbnails.last().expect("could not get thumbnail url").url.clone(),
								username: liveChatMembershipItemRenderer.authorName.simpleText.clone(),
								channel_id: liveChatMembershipItemRenderer.authorExternalChannelId.clone(),
								months: months,
								message: message,
								time: timestring.clone(),
								header_color: i64::from_str_radix("0a8043", 16).expect("somhow failed to parse sponsor color"),
								body_color: i64::from_str_radix("0f9d58", 16).expect("somhow failed to parse sponsor color"),
							};

							donations.push(serde_json::to_string(&membership).expect("could not serialize donation"));

							println!("==========membership end==========");
						},
						ChatItemType::LiveChatSponsorshipsGiftPurchaseAnnouncementRenderer { liveChatSponsorshipsGiftPurchaseAnnouncementRenderer } => {
							num_gifts+=1;
							// gift purchase
							let sep = "=========gifting memberships start=========".black().on_truecolor(15, 157, 88);
							println!("{}",sep);
							//println!("membership gift send: {:#?}", liveChatSponsorshipsGiftPurchaseAnnouncementRenderer);
							
							// print username and channel id
							println!("username: {}, channel: https://youtube.com/channel/{}", liveChatSponsorshipsGiftPurchaseAnnouncementRenderer.header.liveChatSponsorshipsHeaderRenderer.authorName.simpleText, liveChatSponsorshipsGiftPurchaseAnnouncementRenderer.authorExternalChannelId);
							// print number of gifted memberships
							let number_position = liveChatSponsorshipsGiftPurchaseAnnouncementRenderer.header.liveChatSponsorshipsHeaderRenderer.primaryText.runs.get(1);
							let mut num_gifted = String::new();
							match number_position {
								Some(e) => {
									match e {
										RunsTypes::Text { text, italics, bold } => {
											num_gifted = text.to_owned();
											println!("gifted memberships: {}", text);
										},
										_ => {
											println!("error getting number of gifted memberships");
										}
									}
								},
								None=>{}
							};


							let timestring = {
								use chrono::NaiveDateTime;
								let timestamp = liveChatSponsorshipsGiftPurchaseAnnouncementRenderer.timestampUsec;
								let timestamp = timestamp.parse::<i64>().expect("could not parse timestamp");
								let timestamp = timestamp / 1000000;
								let timestamp = NaiveDateTime::from_timestamp_opt(timestamp, 0).expect("could not convert timestamp to datetime");
								timestamp.format("%Y-%m-%d %H:%M:%S").to_string()
							};

							// struct for exporting to json
							#[derive(Serialize, Deserialize, Debug)]
							struct Gift {
								#[serde(rename = "type")]
								json_type: String,
								username: String,
								channel_id: String,
								time: String,
								number: String,
								header_color: i64,
								body_color: i64,
								thumbnail_url: String,
							}

							// thumbnail_url is the last url in the thumbnails array
							// create gifting donation struct
							let gift = Gift {
								json_type: "GiftingMembership".to_string(),
								time: timestring.clone(),
								thumbnail_url: liveChatSponsorshipsGiftPurchaseAnnouncementRenderer.header.liveChatSponsorshipsHeaderRenderer.authorPhoto.thumbnails.last().expect("could not get thumbnail url").url.clone(),
								username: liveChatSponsorshipsGiftPurchaseAnnouncementRenderer.header.liveChatSponsorshipsHeaderRenderer.authorName.simpleText.clone(),
								channel_id: liveChatSponsorshipsGiftPurchaseAnnouncementRenderer.authorExternalChannelId.clone(),
								number: num_gifted,
								header_color: i64::from_str_radix("0a8043", 16).expect("somhow failed to parse sponsor color"),
								body_color: i64::from_str_radix("0f9d58", 16).expect("somhow failed to parse sponsor color"),
							};

							donations.push(serde_json::to_string(&gift).expect("could not serialize donation"));


							println!("==========gifting memberships end==========")

						},
						ChatItemType::LiveChatSponsorshipsGiftRedemptionAnnouncementRenderer { liveChatSponsorshipsGiftRedemptionAnnouncementRenderer } => {
							num_redemptions+=1;
							// message about person who got a gift
							//println!("membership gift receive: {:#?}", liveChatSponsorshipsGiftRedemptionAnnouncementRenderer);

							let sep = "=========membership redemption begins=========".black().on_truecolor(15, 157, 88);
							println!("{}",sep);

							let timestring = match liveChatSponsorshipsGiftRedemptionAnnouncementRenderer.timestampText {
								// should always exist in replays
								Some(timestamp) => timestamp.simpleText,
								// if it doesn't exist fallback to .timestampUsec
								// and convert from microseconds to datetime

								None => {
									use chrono::NaiveDateTime;
									let timestamp = liveChatSponsorshipsGiftRedemptionAnnouncementRenderer.timestampUsec;
									let timestamp = timestamp.parse::<i64>().expect("could not parse timestamp");
									let timestamp = timestamp / 1000000;
									let timestamp = NaiveDateTime::from_timestamp_opt(timestamp, 0).expect("could not convert timestamp to datetime");
									timestamp.format("%Y-%m-%d %H:%M:%S").to_string()
								}
							};

							println!("time: {}", timestring);

							// print username and message
							let mut message = String::new();
							for part in &liveChatSponsorshipsGiftRedemptionAnnouncementRenderer.message.runs {
								match part {
									RunsTypes::Text { text, italics, bold } => {
										message.push_str(&text);
									},
									RunsTypes::Emoji { emoji } => {
										if let Some(is_custom_emoji) = emoji.isCustomEmoji {
											if is_custom_emoji {
												message.push_str(format!(":{}:",&emoji.image.accessibility.accessibilityData.label).as_str());
											} else {
												message.push_str(&emoji.emojiId);
											}
										} else {
											message.push_str(&emoji.emojiId);
										}

										//poll_name.push_str(&emoji.image.accessibility.accessibilityData.label);
									},
									RunsTypes::Unknown(unknown) => {
										println!("UNKNOWN VALUE IN RUNS: {:#?}", unknown);
									},
								}
							}

							#[derive(Serialize, Deserialize, Debug)]
							struct Redemption {
								#[serde(rename = "type")]
								json_type: String,
								thumbnail_url: String,
								username: String,
								channel_id: String,
								time: String,
								sender: String,
								header_color: i64,
								body_color: i64,
							}

							let mut sender = String::from("unknown");
							if let Some(name) = liveChatSponsorshipsGiftRedemptionAnnouncementRenderer.message.runs.into_iter().nth(1) {
								match name {
									RunsTypes::Text { text, italics, bold } => {
										sender = text;
									},
									_ => {}
								}
							}

							// create gifting donation struct
							let redemption = Redemption {
								json_type: "GiftMembership".to_string(),
								thumbnail_url: liveChatSponsorshipsGiftRedemptionAnnouncementRenderer.authorPhoto.thumbnails.last().expect("could not get thumbnail url").url.clone(),
								username: liveChatSponsorshipsGiftRedemptionAnnouncementRenderer.authorName.simpleText.clone(),
								channel_id: liveChatSponsorshipsGiftRedemptionAnnouncementRenderer.authorExternalChannelId.clone(),
								sender: sender,
								time: timestring.clone(),
								header_color: i64::from_str_radix("0a8043", 16).expect("somhow failed to parse sponsor color"),
								body_color: i64::from_str_radix("0f9d58", 16).expect("somhow failed to parse sponsor color"),
							};

							donations.push(serde_json::to_string(&redemption).expect("could not serialize donation"));

							println!("{} {}", liveChatSponsorshipsGiftRedemptionAnnouncementRenderer.authorName.simpleText, message);
							// print recipient channel link
							println!("channel: https://youtube.com/channel/{}", liveChatSponsorshipsGiftRedemptionAnnouncementRenderer.authorExternalChannelId);
							println!("==========membership redemption ends==========")
						},
						ChatItemType::LiveChatPaidStickerRenderer { liveChatPaidStickerRenderer } => {
							num_stickers+=1;
							// liveChatPaidMessageRenderer.bodyBackgroundColor is the raw decimal value of the color
							// convert it to hex
							let background_color = format!("{:x}", liveChatPaidStickerRenderer.backgroundColor);
							// background_color is in argb format with 2 digits for each part
							// split into alpha, red, green, blue
							// this should be safe because the alpha is always ff
							
							assert!(background_color.len() == 8, "background color is not 8 characters long: {}", background_color);

							// split into alpha, red, green, blue u8 values
							let alpha = u8::from_str_radix(&background_color[0..2], 16).expect("could not parse alpha");
							let red = u8::from_str_radix(&background_color[2..4], 16).expect("could not parse red");
							let green = u8::from_str_radix(&background_color[4..6], 16).expect("could not parse green");
							let blue = u8::from_str_radix(&background_color[6..8], 16).expect("could not parse blue");

							let sep = "=========sticker start=========".black().on_truecolor(red, green, blue);
							println!("{}",sep);

							let timestring = match liveChatPaidStickerRenderer.timestampText {
								// should always exist in replays
								Some(timestamp) => timestamp.simpleText,
								// if it doesn't exist fallback to .timestampUsec
								// and convert from microseconds to datetime

								None => {
									use chrono::NaiveDateTime;
									let timestamp = liveChatPaidStickerRenderer.timestampUsec;
									let timestamp = timestamp.parse::<i64>().expect("could not parse timestamp");
									let timestamp = timestamp / 1000000;
									let timestamp = NaiveDateTime::from_timestamp_opt(timestamp, 0).expect("could not convert timestamp to datetime");
									timestamp.format("%Y-%m-%d %H:%M:%S").to_string()
								}
							};
							println!("time: {}", timestring);

							// sticker
							//println!("sticker: {:#?}", liveChatPaidStickerRenderer);

							// print username and channel link
							println!("username: {}, channel: https://youtube.com/channel/{}", liveChatPaidStickerRenderer.authorName.simpleText, liveChatPaidStickerRenderer.authorExternalChannelId);
							// print sticker cost
							println!("sticker cost: {}", liveChatPaidStickerRenderer.purchaseAmountText.simpleText);
							// print sticker description
							println!("sticker description: {}", liveChatPaidStickerRenderer.sticker.accessibility.accessibilityData.label);

							#[derive(Serialize, Deserialize, Debug)]
							struct Sticker {
								#[serde(rename = "type")]
								json_type: String,
								username: String,
								channel_id: String,
								sticker_cost: String,
								sticker_description: String,
								sticker_image_url: String,
								time: String,
								thumbnail_url: String,
								header_color: i64,
								body_color: i64,
							}

							// create gifting donation struct
							let donation = Sticker {
								json_type: "Sticker".to_string(),
								time: timestring.clone(),
								username: liveChatPaidStickerRenderer.authorName.simpleText.clone(),
								channel_id: liveChatPaidStickerRenderer.authorExternalChannelId.clone(),
								sticker_cost: liveChatPaidStickerRenderer.purchaseAmountText.simpleText.clone(),
								sticker_image_url: liveChatPaidStickerRenderer.sticker.thumbnails.last().expect("could not get thumbnail url").url.clone(),
								sticker_description: liveChatPaidStickerRenderer.sticker.accessibility.accessibilityData.label.clone(),
								thumbnail_url: liveChatPaidStickerRenderer.authorPhoto.thumbnails.last().expect("could not get thumbnail url").url.clone(),
								header_color: i64::from_str_radix(&background_color, 16).expect("somhow failed to parse sponsor color"),
								body_color: i64::from_str_radix(&background_color, 16).expect("somhow failed to parse sponsor color"),
							};

							donations.push(serde_json::to_string(&donation).expect("could not serialize donation"));

							println!("==========sticker end==========")
						},



						ChatItemType::LiveChatTextMessageRenderer { liveChatTextMessageRenderer } => {
							// normal message
							num_messages += 1;
							//println!("live_chat_text_message_renderer: {:#?}", liveChatTextMessageRenderer);
						}, 
						ChatItemType::LiveChatPlaceholderItemRenderer { liveChatPlaceholderItemRenderer } => {
							// placeholder with no content
							//println!("live_chat_placeholder_item_renderer: {:#?}", liveChatPlaceholderItemRenderer);
						},
						ChatItemType::LiveChatViewerEngagementMessageRenderer { liveChatViewerEngagementMessageRenderer } => {
							// message about subscriber mode only and possibly other things
							//println!("live_chat_viewer_engagement_message_renderer: {:#?}", liveChatViewerEngagementMessageRenderer);
						},
						ChatItemType::LiveChatModeChangeMessageRenderer { liveChatModeChangeMessageRenderer } => {
							// message about slow mode, members only mode, etc.
							//println!("live_chat_mode_change_message_renderer: {:#?}", liveChatModeChangeMessageRenderer);
						},

						ChatItemType::Unknown(s) => {
							println!("UNKNOWN TYPE OF CHAT MESSAGE PLEASE REPORT: {:#?}", s);
						}
					}

					//println!("add_chat_item_action: {:#?}", addChatItemAction.item);
				},
				Action::AddBannerToLiveChatCommand { addBannerToLiveChatCommand, clickTrackingParams } => {
					//println!("add_banner_to_live_chat_command: {}", addBannerToLiveChatCommand);
				},
				Action::AddLiveChatTickerItemAction { addLiveChatTickerItemAction, clickTrackingParams } => {
					// youtube join button and donations larger then 5$ are here
					//println!("add_live_chat_ticker_item_action: {}", addLiveChatTickerItemAction);
				},
				Action::CloseLiveChatActionPanelAction { closeLiveChatActionPanelAction, clickTrackingParams } => {
					//println!("close_live_chat_action_panel_action: {}", closeLiveChatActionPanelAction);
				},
				Action::LiveChatReportModerationStateCommand { liveChatReportModerationStateCommand, clickTrackingParams } => {
					//println!("live_chat_report_moderation_state_command: {}", liveChatReportModerationStateCommand);
				},
				Action::RemoveBannerForLiveChatCommand { removeBannerForLiveChatCommand, clickTrackingParams } => {
					//println!("remove_banner_for_live_chat_command: {}", removeBannerForLiveChatCommand);
				},
				Action::RemoveChatItemAction { removeChatItemAction, clickTrackingParams } => {
					//println!("remove_chat_item_action: {}", removeChatItemAction);
				},
				Action::RemoveChatItemByAuthorAction { removeChatItemByAuthorAction, clickTrackingParams } => {
					//println!("remove_chat_item_by_author_action: {}", removeChatItemByAuthorAction);
				},
				Action::ReplaceChatItemAction { replaceChatItemAction, clickTrackingParams } => {
					//println!("replace_chat_item_action: {}", replaceChatItemAction);
				},
				Action::ShowLiveChatActionPanelAction { showLiveChatActionPanelAction, clickTrackingParams } => {
					//println!("show_live_chat_action_panel_action: {}", showLiveChatActionPanelAction);
				},
				Action::UpdateLiveChatPollAction { updateLiveChatPollAction, clickTrackingParams } => {
					// handle poll events
					continue; // disable poll events for now

					println!("==========poll update==========");
					// print name of poll
					let mut poll_name = String::new();
					for run in updateLiveChatPollAction.pollToUpdate.pollRenderer.header.pollHeaderRenderer.pollQuestion.runs {
						match run {
							RunsTypes::Text { text, italics, bold } => {
								poll_name.push_str(&text);
							},
							RunsTypes::Emoji { emoji } => {
								println!("emoji: {:?}", emoji)

								//poll_name.push_str(&emoji.image.accessibility.accessibilityData.label);
							},
							RunsTypes::Unknown(unknown) => {
								println!("unknown: {:?}", unknown);
							},

						}
					}
					println!("poll question: {}", poll_name);

					// print metadata
					let mut metadata = String::new();
					for run in updateLiveChatPollAction.pollToUpdate.pollRenderer.header.pollHeaderRenderer.metadataText.runs {
						match run {
							RunsTypes::Text { text, italics, bold } => {
								metadata.push_str(&text);
							},
							RunsTypes::Emoji { emoji } => {
								println!("emoji: {:?}", emoji)

								//poll_name.push_str(&emoji.image.accessibility.accessibilityData.label);
							},
							RunsTypes::Unknown(unknown) => {
								println!("unknown: {:?}", unknown);
							},
						}
					}
					println!("metadata: {}", metadata);

					


					// print choices
					for choice in updateLiveChatPollAction.pollToUpdate.pollRenderer.choices {
						let mut choice_name = String::new();
						for run in choice.text.runs {
							match run {
								RunsTypes::Text { text, italics, bold } => {
									choice_name.push_str(&text);
								},
								RunsTypes::Emoji { emoji } => {
									println!("emoji: {:?}", emoji)

									//poll_name.push_str(&emoji.image.accessibility.accessibilityData.label);
								},
								RunsTypes::Unknown(unknown) => {
									println!("unknown: {:?}", unknown);
								},
							}
						}
						println!("choice: {}", choice_name);
						println!("vote percentage: {}", choice.votePercentage.simpleText);
					}
					
					
					println!("==========poll update end==========");

				},
				Action::ReplaceLiveChatRendererAction { replaceLiveChatRendererAction, clickTrackingParams } => {
					//println!("replace_live_chat_renderer_action: {}", replaceLiveChatRendererAction);
				},

				Action::Unknown(value) => {
					println!("unknown action: {:#?}", value);
				}
			}
		}
	}

	
	println!("stats:");
	// total messages
	println!("total messages: {}", num_messages);
	// total superchats
	println!("total superchats: {}", num_superchats);
	// total memberships
	println!("total memberships: {}", num_memberships);
	// total gifts
	println!("total gifts: {}", num_gifts);
	// total redemptions
	println!("total redemptions: {}", num_redemptions);
	// total stickers
	println!("total stickers: {}", num_stickers);

	// messages to superchats ratio
	println!("messages to superchats ratio: {}", num_messages as f32 / num_superchats as f32);

	// average gift amount
	println!("average gift amount: {}", num_redemptions as f32 / num_gifts as f32);

	// if args.outputfile then write all donations to file
	if let Some(outputfile) = args.outputfile {
		let mut file = std::fs::File::create(outputfile).unwrap();
		for donation in donations {
			file.write_all(format!("{}\n", donation).as_bytes()).unwrap();
		}
	}

}
