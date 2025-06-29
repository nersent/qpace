import { createWriteStream } from "node:fs";
import { pipeline, Writable } from "stream";
import * as stream from "stream";
import { promisify } from "util";

import axios, { AxiosInstance, AxiosPromise, AxiosRequestConfig } from "axios";
import * as http2 from "http2-wrapper";

export const axiosHttp2Adapter = (config: AxiosRequestConfig): AxiosPromise => {
  let req: http2.ClientRequest | undefined = undefined;
  (config as any).transport = {
    request: function request(options: any, handleResponse: any): any {
      req = http2.request(options, handleResponse);
      return req;
    },
  };
  const ret = (axios.defaults.adapter as any)!(config);
  // Remove the axios action `socket.setKeepAlive` because the HTTP/2 sockets should not be directly manipulated
  const listeners = req!.listeners("socket");
  if (listeners.length) req!.removeListener("socket", listeners[0] as any);
  return ret;
};

const _pipeline = promisify(pipeline);

export const downloadFile = async (
  url: string,
  writeStream: Writable,
  axiosInstance: AxiosInstance = axios,
): Promise<void> => {
  const req = await axiosInstance({
    method: "get",
    url,
    responseType: "stream",
  });

  await _pipeline(req.data, writeStream);
};

export const downloadFileToPath = async (
  url: string,
  path: string,
  axiosInstance?: AxiosInstance,
): Promise<void> => {
  // const writeStream = createWriteStream(path);
  // await downloadFile(url, writeStream);
  const finishedDownload = promisify(stream.finished);
  const writer = createWriteStream(path, {
    autoClose: true,
    flags: "w",
  });

  // content-type is application/octet-stream"
  const response = await (axiosInstance ?? axios)({
    method: "GET",
    url: url,
    responseType: "stream",
    maxBodyLength: Infinity,
  });

  response.data.pipe(writer);
  await finishedDownload(writer);
};
