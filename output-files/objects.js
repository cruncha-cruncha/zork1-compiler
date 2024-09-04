export const objects = {
  torch: {},
  pedestal: {},
};

export const lookupBySynonym = (word) => {
  switch (word) {
  case 'PEDESTAL':
    return objects['pedestal'];
  case 'TORCH':
  case 'IVORY':
  case 'TREASURE':
    return objects['torch'];
  default:
    return null;
  }
}
