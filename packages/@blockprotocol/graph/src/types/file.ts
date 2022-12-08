import { EntityId } from "./entity";

export type FileMediaType = "image" | "video";

export type UploadFileData = {
  file?: File | null;
  url?: string | null;
  mediaType: FileMediaType;
};

export type UploadFileReturn = {
  entityId: EntityId;
  url: string;
  mediaType: FileMediaType;
};
