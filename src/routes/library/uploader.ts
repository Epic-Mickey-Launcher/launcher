export namespace Uploader {
  const chunkSize = 20; //MB

  export function FileUpload(file: File) {
    const chunkSizeBytes = chunkSize * 1024 * 1024;

    const chunkParts = Math.ceil(file.size / chunkSizeBytes);

    console.log(`Chopping up file into ${chunkParts} ${chunkSize}MB Parts`);

    let curPointer = 0;

    let chunks = [];

    while (curPointer < file.size) {
      chunks.push(file.slice(curPointer, curPointer + chunkSizeBytes));
      curPointer += chunkSizeBytes;
    }
  }
}
