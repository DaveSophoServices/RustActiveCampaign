use serde::{Deserialize,Deserializer,Serialize};
use std::collections::HashMap;

#[derive(Serialize,Deserialize,Debug)]
pub struct Campaign {
    // "type": "single",
    #[serde(alias="type")]
    ctype: String,
    // "userid": "1",
    userid: String,
    // "segmentid": "0",
    segmentid: String,
    // "bounceid": "-1",
    bounceid: String,
    // "realcid": "0",
    realcid: String,
    // "sendid": "0",
    sendid: String,
    // "threadid": "0",
    threadid: String,
    // "seriesid": "0",
    seriesid: String,
    // "formid": "1",
    formid: String,
    // "basetemplateid": "90a1d1ab6ad33e95708caf6472dc1724f698c694",
    basetemplateid: String,
    // "basemessageid": "0",
    basemessageid: String,
    // "addressid": "0",
    addressid: String,
    // "source": "web",
    source: String,
    // "name": "Opt In Email",
    name: String,
    // "cdate": "2018-08-17T13:47:31-05:00",
    cdate: String,
    // "mdate": "2018-08-17T13:47:31-05:00",
    mdate: Option<String>,
    // "sdate": null,
    sdate: Option<String>,
    // "ldate": null,
    ldate: Option<String>,
    // "send_amt": "0",    
    #[serde(deserialize_with="deserialize_u64")]
    send_amt: u64,
    #[serde(deserialize_with="deserialize_u64")]
    // "total_amt": "0",
    total_amt: u64,
    // "opens": "0",
    #[serde(deserialize_with="deserialize_u64")]
    opens: u64,
    // "uniqueopens": "0",
    #[serde(deserialize_with="deserialize_u64")]
    uniqueopens: u64,
    // "linkclicks": "0",
    #[serde(deserialize_with="deserialize_u64")]
    linkclicks: u64,
    // "uniquelinkclicks": "0",
    #[serde(deserialize_with="deserialize_u64")]
    uniquelinkclicks: u64,
    // "subscriberclicks": "0",
    #[serde(deserialize_with="deserialize_u64")]
    subscriberclicks: u64,
    // "forwards": "0",
    #[serde(deserialize_with="deserialize_u64")]
    forwards: u64,
    // "uniqueforwards": "0",
    #[serde(deserialize_with="deserialize_u64")]
    uniqueforwards: u64,
    // "hardbounces": "0",
    #[serde(deserialize_with="deserialize_u64")]
    hardbounces: u64,
    // "softbounces": "0",
    #[serde(deserialize_with="deserialize_u64")]
    softbounces: u64,
    // "unsubscribes": "0",
    #[serde(deserialize_with="deserialize_u64")]
    unsubscribes: u64,
    // "unsubreasons": "0",
    #[serde(deserialize_with="deserialize_u64")]
    unsubreasons: u64,
    // "updates": "0",
    #[serde(deserialize_with="deserialize_u64")]
    updates: u64,
    // "socialshares": "0",
    #[serde(deserialize_with="deserialize_u64")]
    socialshares: u64,
    // "replies": "0",
    #[serde(deserialize_with="deserialize_u64")]
    replies: u64,
    // "uniquereplies": "0",
    #[serde(deserialize_with="deserialize_u64")]
    uniquereplies: u64,
    // "status": "0",
    status: String,
    // "public": "1",
    public: String,
    // "mail_transfer": "0",
    mail_transfer: String,
    // "mail_send": "0",
    mail_send: String,
    // "mail_cleanup": "0",
    mail_cleanup: String,
    // "mailer_log_file": "0",
    mailer_log_file: String,
    // "tracklinks": "all",
    tracklinks: String,
    // "tracklinksanalytics": "0",
    tracklinksanalytics: String,
    // "trackreads": "1",
    trackreads: String,
    // "trackreadsanalytics": "1",
    trackreadsanalytics: String,
    // "analytics_campaign_name": "",
    analytics_campaign_name: String,
    // "tweet": "0",
    tweet: String,
    // "facebook": "0",
    facebook: String,
    // "survey": "",
    survey: String,
    // "embed_images": "0",
    embed_images: String,
    // "htmlunsub": "0",
    htmlunsub: String,
    // "textunsub": "0",
    textunsub: String,
    // "htmlunsubdata": null,
    htmlunsubdata: Option<String>,
    // "textunsubdata": null,
    textunsubdata: Option<String>,
    // "recurring": "day1",
    recurring: String,
    // "willrecur": "0",
    willrecur: String,
    // "split_type": "even",
    split_type: String,
    // "split_content": "0",
    split_content: String,
    // "split_offset": "2",
    split_offset: String,
    // "split_offset_type": "day",
    split_offset_type: String,
    // "split_winner_messageid": "0",
    split_winner_messageid: String,
    // "split_winner_awaiting": "0",
    split_winner_awaiting: String,
    // "responder_offset": "0",
    responder_offset: String,
    // "responder_type": "subscribe",
    responder_type: String,
    // "responder_existing": "0",
    responder_existing: String,
    // "reminder_field": "sdate",
    reminder_field: String,
    // "reminder_format": null,
    reminder_format: Option<String>,
    // "reminder_type": "month_day",
    reminder_type: String,
    // "reminder_offset": "0",
    reminder_offset: String,
    // "reminder_offset_type": "day",
    reminder_offset_type: String,
    // "reminder_offset_sign": "+",
    reminder_offset_sign: String,
    // "reminder_last_cron_run": null,
    reminder_last_cron_run: Option<String>,
    // "activerss_interval": "day1",
    activerss_interval: String,
    // "activerss_url": null,
    activerss_url: Option<String>,
    // "activerss_items": "10",
    activerss_items: String,
    // "ip4": "643992596",
    ip4: String,
    // "laststep": "designer",
    laststep: String,
    // "managetext": "0",
    managetext: String,
    // "schedule": "0",
    schedule: String,
    // "scheduleddate": null,
    scheduleddate: Option<String>,
    // "waitpreview": "0",
    waitpreview: String,
    // "deletestamp": null,
    deletestamp: Option<String>,
    // "replysys": "0",
    replysys: String,
    // "links": {
    //     "user": "https://:account.api-us1.com/api/3/campaigns/1/user",
    //     "automation": "https://:account.api-us1.com/api/3/campaigns/1/automation",
    //     "campaignMessage": "https://:account.api-us1.com/api/3/campaigns/1/campaignMessage",
    //     "campaignMessages": "https://:account.api-us1.com/api/3/campaigns/1/campaignMessages",
    //     "links": "https://:account.api-us1.com/api/3/campaigns/1/links",
    //     "campaignLists": "https://:account.api-us1.com/api/3/campaigns/1/campaignLists"
    // },
    links: HashMap<String,String>,
    // "id": "1",
    id: String,
    // "user": "1",
    user: String,
    // "automation": null
    automation: Option<String>,
}
#[derive(Deserialize,Debug)]
pub struct Campaigns {
    pub campaigns: Vec<Campaign>,
    pub meta: super::Meta,
}

fn deserialize_u64<'de,D>(deserializer:D) -> Result<u64, D::Error>
where D: Deserializer<'de> {
    let buf = String::deserialize(deserializer)?;
    //println!("mapping {} -> {:#?}", buf, buf.parse::<u64>());
    buf.parse::<u64>().map_err(serde::de::Error::custom)
}

// impl Campaign {
//     pub fn to_string(&self) -> String {
// 	format!("{ctype},{userid},{segmentid},{bounceid},{realcid},{sendid},\
// 		 {threadid},{seriesid},{formid},{basetemplateid},\
// 		 {basemessageid},{addressid},{source},{name},{cdate},{mdate},\
// 		 {sdate},{ldate},{send_amt},{total_amt},{opens},{uniqueopens},\
// 		 {linkclicks},{uniquelinkclicks},{subscriberclicks},{forwards},\
// 		 {uniqueforwards},{hardbounces},{softbounces},{unsubscribes},\
// 		 {unsubscribes},{unsubreasons},{updates},{socialshares},\
// 		 {replies},{uniquereplies}
// 		 ", self.ctype, self.userid, self.segmentid, self.bounceid,
// 		self.realcid, self.sendid, self.threadid, self.seriesid,
// 		self.formid, self.basetemplateid, self.basemessageid,
// 		self.addressid, self.source, self.name, self.cdate,
// 		self.mdate, self.sdate,	self.ldate, self.send_amt,
// 		self.total_amt, self.opens, self.uniqueopens, self.linkclicks,
// 		self.uniquelinkclicks, self.subscriberclicks, self.forwards,
// 		self.uniqueforwards, self.hardbounces, self.softbounces,
// 		self.unsubscribes, self.unsubreasons, self.updates,
// 		self.socialshares, self.replies, self.uniquereplies,
// 		self.status, self.public, self.mail_transfer, self.mail_send,
// 		self.mail_cleanup, self.mailer_log_file, self.tracklinks,
// 		self.tracklinksanalytics, self.trackreads,
// 		self.trackreadsanalytics, self.analytics_campaign_name,
// 		self.tweet, self.facebook, self.survey, self.embed_images,
// 		self.htmlunsub, self.textunsub, self.htmlunsubdata,
// 		self.textunsubdata, self.recurring, self.willrecur,
// 		self.split_type, self.split_content, self.split_offset,
// 		self.split_offset_type, self.split_winner_messageid,
// 		self.split_winner_awaiting, self.responder_offset,
// 		self.responder_type, self.responder_existing,
// 		self.reminder_field, self.reminder_format,
// 		self.reminder_type, self.reminder_offset,
// 		self.reminder_offset_type, self.reminder_offset_sign,
// 		self.reminder_last_cron_run,
// 		self.activerss_interval, self.activerss_url,
// 		self.activerss_items, self.ip4, self.laststep,
// 		self.managetext, self.schedule, self.scheduleddate,
// 		self.waitpreview, self.deletestamp, self.replysys,
// 		self.id, self.user, self.automation
// 	);
//     }
// }
