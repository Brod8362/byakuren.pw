use rocket::time::{OffsetDateTime, format_description::well_known::Rfc2822};

use crate::post::PostInfo;

static RSS_HEADER: &str = 
"<?xml version=\"1.0\" encoding=\"UTF-8\" ?>
<rss version=\"2.0\">
<channel>
<title>byakuren.pw</title>
<link>https://hijiri.byakuren.pw</link>
<description>byakuren.pw blog feed</description>
";
static RSS_FOOTER: &str = "</channel></rss>";

pub fn generate_rss(posts: &Vec<PostInfo>, domain: &str) -> String {
    let mut xml = vec![String::from(RSS_HEADER)];
    for post in posts {
        xml.push(generate_post_element(post, domain));
    }
    xml.push(String::from(RSS_FOOTER));
    xml.join("")
}

fn generate_post_element(post: &PostInfo, domain: &str) -> String {
    let offset_time = OffsetDateTime::from_unix_timestamp(post.timestamp).expect("failed to parse datetime");
    format!("
    <item>
        <title>{}</title>
        <link>{}/page/{}</link>
        <guid isPermaLink=\"false\">{}</guid>
        <pubDate>{}</pubDate>
    </item>
    ", post.title, domain, post.id, post.id, offset_time.format(&Rfc2822).expect("failed to format post timestamp"))
}