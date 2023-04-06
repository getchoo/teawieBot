use include_dir::{include_dir, Dir};
use rand::seq::SliceRandom;
use std::collections::HashMap;
use std::vec;

const CHAR_LIMIT: usize = 2000;
const FILES: Dir = include_dir!("src/include");

pub const TEAMOJIS: [&str; 15] = [
	"<:teawiecry:1056438041872433303>",
	"<:teawiederp:1056438043109757018>",
	"<:teawiedizzy:1056438044149960794>",
	"<:teawienerdcroppedanddownsized:1056438150123233320>",
	"<:teawieneutral:1056438044930084928>",
	"<a:teawiepet:1056438324392374400>",
	"<a:teawiepetfast:1056438325445136414>",
	"<a:teawiepop:1066240491806531625>",
	"<:teawiesmile:1056438046440042546>",
	"<:teawiesmug:1056438047362781216>",
	"<:teawiestarstruck:1056438048721744022>",
	"<:tei:1066249455281655908>",
	"<:wavy:1066518302974812180>",
	"<:wie:1066249592489902080>",
	"<:manythoughtsheadfull:1065887153399283742>",
];

const RESPONSES: [&str; 20] = [
	"soon",
	"maybe",
	"perhaps",
	"elaborate",
	"Twitter's Recommendation Algorithm",
	// i was lazy here
	"<:teawiecry:1056438041872433303>",
	"<:teawiederp:1056438043109757018>",
	"<:teawiedizzy:1056438044149960794>",
	"<:teawienerdcroppedanddownsized:1056438150123233320>",
	"<:teawieneutral:1056438044930084928>",
	"<a:teawiepet:1056438324392374400>",
	"<a:teawiepetfast:1056438325445136414>",
	"<a:teawiepop:1066240491806531625>",
	"<:teawiesmile:1056438046440042546>",
	"<:teawiesmug:1056438047362781216>",
	"<:teawiestarstruck:1056438048721744022>",
	"<:tei:1066249455281655908>",
	"<:wavy:1066518302974812180>",
	"<:wie:1066249592489902080>",
	"<:manythoughtsheadfull:1065887153399283742>",
];

const LORE: [&str; 19] = [
	"Teawie is made of silly string and blood.",
	"Teawie has the mentality of an eight year old, but no discernable age.",
	"Teawie can change size to fit into tight spaces. Although prefers to be small most of the time.",
	"Teawie is blue flavored. The color specifically.",
	"Teawie produces asexually via mitosis and in other cells.",
	"Teawie cannot be physically damaged, and if put into a blender of some sort, will become stretchy.",
	"Teawie has one tooth, but originally had a full set. These were lost due to him constantly eating stuff he probably shouldn't, such as computer hardware.",
	"Teawie smells like cotton candy and fiberglass insulation.",
	"Teawie at a base \"knowledge level\", can only repeat his name. However this can be changed if Teawie inherets \"strings\" of information from a host.",
	"Teawie can \"infect\" other cells and turn them into Teawie.",
	"Teawie has a gun, but the only bullets it has is smaller Teawies. They have the same attributes as Teawie, so they perform mitosis to keep the clip stocked.",
	"The gun resembles the Ray Gun from the Call of Duty series. However this gun is blue rather than red.",
	"Teawie \"bullets\" typically lodge into parts of a body and proceed to infect the victim. If a Teawie ends up lodged into the brain, Teawie will be inside the mind of the user, reigning pure terror until the victim passes on. Teawie will pilot the victim like a meat suit until decomposition fully occurs, and then find a new host.",
	"A Teawie \"bullet\" will usually kill the host upon impact, However if the host survives, they will exhibit symptoms such as lightheadedness, fainting, fatigue, delusions, mental confusion and decline, disorientation, hallucinations and eventually death.",
	"If a host possesses particular \"strings\" of information that a Teawie does not currently possess, Teawie will add those current strings to himself once the host expires. Essentially giving Teawie the host's memories and knowledge. This starts to debilitate overtime. Eventually ending right back where the Teawie started.",
	"Teawie is capable of infecting non-organic material such as a Personal Computer. If infecting something with quite a bit of power, Teawie will inheret \"strings\" from the object and become more powerful as a result.",
	"Teawie himself isnt radioactive however some Teawies have a pretty bad habit of being in or around places with incredibly high radiation.",
	"Teawie has a Go-Kart that he will sometimes drive around in, however due to being mentally eight years old at a base \"knowledge level\", is not good at all with driving, however despite numerous crashes, ends up unscathed.",
	"Teawie will \"insert\" himself into various media while trying to move from host to host.",
];

/*
 * chooses a random response out of our options
 */
pub async fn get_random_response() -> String {
	let mut rng = rand::thread_rng();
	let resp = RESPONSES
		.choose(&mut rng)
		.expect("couldn't choose random value!");
	resp.to_string()
}

/*
 * splits a message into multiple parts so that
 * it can fit discord's character limit
 */
fn split_msg(msg: &String) -> Vec<String> {
	if msg.len() > CHAR_LIMIT {
		let split = msg[CHAR_LIMIT..].to_string();

		let add = split_msg(&split);
		let mut ret = vec![msg[..CHAR_LIMIT].to_string()];

		for v in add {
			ret.push(v);
		}

		return ret;
	}
	vec![msg.to_string()]
}

/*
 * gets a random copypasta from include/
 */
pub async fn get_copypasta(name: &str) -> Vec<String> {
	let mut files: HashMap<&str, &str> = HashMap::new();

	for file in FILES.files() {
		let name = file.path().file_stem().unwrap().to_str().unwrap();

		let contents = file.contents_utf8().unwrap();

		// refer to files by their name w/o extension
		files.insert(name, contents);
	}

	if files.contains_key(&name) {
		let reply = files[name];
		// split message if it's too big
		if reply.len() > CHAR_LIMIT {
			return split_msg(&reply.to_string());
		}
		return vec![reply.to_string()];
	}

	let err = format!("couldn't find {:?} in files", name);
	vec![err]
}
