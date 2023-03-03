use regex::Regex;
use tracing::info;

/// The Blocklist which is shared across the application and holds a list of IP addresses as Strings
#[derive(Default, Debug)]
pub struct Blocklist(pub Vec<String>);

/// Querying the GitHub endpoint and fetching the file, extracting the IP addresses from it and returning
/// it as a a type `Blocklist`
pub async fn fetch_latest_blocklist() -> Blocklist {
	info!("Fetch latest blocklist");

	let resp = reqwest::get("https://raw.githubusercontent.com/stamparm/ipsum/master/ipsum.txt")
		.await
		.unwrap()
		.text()
		.await
		.unwrap();

	let regex = Regex::new(
		r"\b(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\b",
	)
	.unwrap();

	let mut res: Vec<String> = regex
		.find_iter(&resp)
		.map(|mat| mat.as_str().to_string())
		.collect();

	res.sort();

	Blocklist(res)
}
