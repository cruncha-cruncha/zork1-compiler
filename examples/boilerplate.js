// local() and global() always return an object like { isLocal, name, val }
// set() accepts anything and wraps it in an object

const EMPTY_GLOBAL = {
	isLocal = false,
	name = "",
	val = null
}

const globalLookup = {};

const setg = (vName, value) => {
	if (typeof vName !== "string") {
		throw(`Bad vName: "${vName}", during setg, with value "${value}"`);
	}
	globalLookup[vName] = {
		isLocal: false,
		name: vName,
		val: ("val" in value) ? value.val : value
	};
	return value;
}

const global = (v) => {
	if (typeof v !== "string") {
		if (typeof v === "object" && "name" in v) {
			v = v.name;
		} else {
			throw(`Bad arg to local: "${v}"`)
		}
	}
	const out = globalLookup[v];
	return out ? out : EMPTY_GLOBAL;
}

const EMPTY_LOCAL = {
	isLocal = true,
	name = "",
	val = null
};

const newLocalScope = (fn) => {
	const lookup = {};

	const set = (vName, value) => {
		if (typeof vName !== "string") {
			throw(`Bad vName: "${vName}", during set, with value "${value}"`);
		}
		lookup[vName] = {
			isLocal: true,
			name: vName,
			val: ("val" in value) ? value.val : value
		};
		return value;
	}

	const local = (v) => {
		if (typeof v === "object" && "name" in v) {
			v = v.name;
		} else if (typeof v !== string) {
			throw(`Bad arg to local: "${v}"`)
		}
		const out = lookup[v];
		return out ? out : EMPTY_LOCAL;
	}
  
	return fn;
}