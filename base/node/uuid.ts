const { v4: _uuidv4 } = require("uuid");

export const uuidv4 = (): string => _uuidv4();
