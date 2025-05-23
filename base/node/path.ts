export const removeExtensionDot = (ext: string): string => {
  let i = ext.indexOf(".");
  if (i === -1) return ext;
  return ext.slice(i + 1);
};

export const removeExtension = (filename: string): string => {
  const dotIndex = filename.lastIndexOf(".");
  if (dotIndex === -1) {
    return filename;
  }
  return filename.split(".").slice(0, -1).join(".");
};

export const withoutExt = removeExtension;
