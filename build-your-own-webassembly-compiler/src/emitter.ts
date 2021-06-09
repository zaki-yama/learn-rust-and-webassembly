const magicModuleHeader = [0x00, 0x61, 0x73, 0x6d]; // \0asm
const moduleVersion = [0x01, 0x00, 0x00, 0x00];

export const emitter: Emitter = () => {
  return Uint8Array.from([...magicModuleHeader, ...moduleVersion]);
};
