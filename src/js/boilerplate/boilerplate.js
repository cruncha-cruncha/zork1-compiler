// local() and global() always return an object like { isLocal, name, val }
// set() accepts anything and wraps it in an object

const EMPTY_GLOBAL = {
  scope: "global",
  name: "",
  val: null,
};

const globalLookup = {};

const setg = (vName, value) => {
  if (typeof vName !== "string") {
    throw `Bad vName: "${vName}", during setg, with value "${value}" and type "${typeof value}"`;
  }

  if (vName in globalLookup) {
    throw `Variable "${vName}" already exists in global scope`;
  }

  globalLookup[vName] = {
    name: vName,
    val: value, // "val" in value ? value.val : value,
  };

  return value;
};

const getGlobal = (v) => {
  //   if (typeof v === "object" && "name" in v) {
  //     v = v.name;
  //   } else
  if (typeof v !== "string") {
    throw `Bad arg to global: "${v}" of type "${typeof v}"`;
  }

  if (!(v in globalLookup)) {
    throw `Variable "${v}" not found in global scope`;
  }

  return globalLookup[v];
};

const EMPTY_LOCAL = {
  scope: "local",
  name: "",
  val: null,
};

const newLocalScope = (fn) => {
  const localLookup = {};

  const set = (vName, value) => {
    if (typeof vName !== "string") {
      throw `Bad vName: "${vName}", during set, with value "${value}" and type "${typeof value}"`;
    }

    if (vName in localLookup) {
      throw `Variable "${vName}" already exists in local scope`;
    }

    localLookup[vName] = {
      name: vName,
      val: value, // "val" in value ? value.val : value,
    };

    return value;
  };

  const getLocal = (v) => {
    // if (typeof v === "object" && "name" in v) {
    //   v = v.name();
    // } else
    if (typeof v !== string) {
      throw `Bad arg to local: "${v}" of type "${typeof v}"`;
    }

    if (!(out in localLookup)) {
      throw `Variable "${v}" not found in local scope`;
    }

    return lookup[v];
  };

  return fn;
};
