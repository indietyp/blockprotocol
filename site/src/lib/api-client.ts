import axios, { AxiosError, AxiosRequestConfig, AxiosResponse } from "axios";
import { ValidationError } from "express-validator";

import { ApiBlockSearchResponse } from "../pages/api/blocks.api";
import {
  ApiLoginWithLoginCodeRequestBody,
  ApiLoginWithLoginCodeResponse,
} from "../pages/api/login-with-login-code.api";
import { ApiKeysResponse } from "../pages/api/me/api-keys.api";
import {
  ApiGenerateApiKeyBody,
  ApiGenerateApiKeyResponse,
} from "../pages/api/me/generate-api-key.api";
import {
  ApiSendLoginCodeRequestBody,
  ApiSendLoginCodeResponse,
} from "../pages/api/send-login-code.api";
import {
  ApiSignupRequestBody,
  ApiSignupResponse,
} from "../pages/api/signup.api";
import {
  ApiTypeUpdateRequest,
  ApiTypeUpdateResponse,
} from "../pages/api/types/[id]/update.api";
import {
  ApiTypeCreateRequest,
  ApiTypeCreateResponse,
} from "../pages/api/types/create.api";
import { ApiUserByShortnameResponse } from "../pages/api/users/[shortname].api";
import { ApiBlocksByUserResponse } from "../pages/api/users/[shortname]/blocks/index.api";
import { ApiTypeByUserAndTitleResponse } from "../pages/api/users/[shortname]/types/[title].api";
import { ApiTypesByUserResponse } from "../pages/api/users/[shortname]/types/index.api";
import {
  ApiVerifyEmailRequestBody,
  ApiVerifyEmailResponse,
} from "../pages/api/verify-email.api";
import { FRONTEND_URL } from "./config";

const BASE_URL = `${FRONTEND_URL}/api/`;

const axiosClient = axios.create({
  baseURL: BASE_URL,
  withCredentials: true,
});

export type ApiClientError = AxiosError<{
  errors?: Partial<ValidationError>[];
}>;

const parseErrorMessageFromAxiosError = (error: ApiClientError): string => {
  const firstValidationErrorMessage = error.response?.data.errors?.find(
    ({ msg }) => !!msg,
  )?.msg;

  return (
    firstValidationErrorMessage ??
    error.response?.statusText ??
    "An error occurred"
  );
};

const handleAxiosError = (
  axiosError: ApiClientError,
): { error: ApiClientError } => {
  /** @todo: report unexpected server errors to sentry or equivalent */
  const error = {
    ...axiosError,
    message: parseErrorMessageFromAxiosError(axiosError),
  };
  return { error };
};

const get = <ResponseData = any, RequestParams = any>(
  url: string,
  requestParams?: RequestParams,
  requiresApiKey?: boolean,
): Promise<{
  data?: ResponseData;
  error?: ApiClientError;
}> => {
  if (requiresApiKey && typeof window !== "undefined") {
    throw new Error(
      `Request to ${url} requires an API key and can only be called on a server render`,
    );
  }
  const apiKey = process.env.BLOCK_PROTOCOL_API_KEY;
  if (requiresApiKey && !apiKey) {
    throw new Error(
      `Request to ${url} requires BLOCK_PROTOCOL_API_KEY in environment`,
    );
  }

  const headers = requiresApiKey ? { "x-api-key": apiKey! } : undefined;

  return axiosClient
    .get<ResponseData>(url, {
      headers,
      params: requestParams,
    })
    .then(({ data }) => ({ data }))
    .catch(handleAxiosError);
};

const post = <RequestData = any, ResponseData = any>(
  url: string,
  requestData?: RequestData,
  config?: AxiosRequestConfig<RequestData>,
): Promise<{
  data?: ResponseData;
  error?: ApiClientError;
}> =>
  axiosClient
    .post<ResponseData, AxiosResponse<ResponseData>, RequestData>(
      url,
      requestData,
      config,
    )
    .then(({ data }) => ({ data }))
    .catch(handleAxiosError);

const put = <RequestData = any, ResponseData = any>(
  url: string,
  requestData?: RequestData,
  config?: AxiosRequestConfig<RequestData>,
): Promise<{
  data?: ResponseData;
  error?: ApiClientError;
}> =>
  axiosClient
    .put<ResponseData, AxiosResponse<ResponseData>, RequestData>(
      url,
      requestData,
      config,
    )
    .then(({ data }) => ({ data }))
    .catch(handleAxiosError);

export const apiClient = {
  get,
  post,
  put,
  generateApiKey: (requestData: ApiGenerateApiKeyBody) =>
    apiClient.post<ApiGenerateApiKeyBody, ApiGenerateApiKeyResponse>(
      "me/generate-api-key",
      requestData,
    ),
  getBlocks: () => apiClient.get<ApiBlockSearchResponse>("/blocks", {}, true),
  getUserApiKeys: () => apiClient.get<ApiKeysResponse>("me/api-keys"),
  getUser: ({ shortname }: { shortname: string }) =>
    apiClient.get<ApiUserByShortnameResponse>(`users/${shortname}`),
  getUserBlocks: ({ shortname }: { shortname: string }) =>
    apiClient.get<ApiBlocksByUserResponse>(`users/${shortname}/blocks`),
  getUserEntityTypes: ({ shortname }: { shortname: string }) =>
    apiClient.get<ApiTypesByUserResponse>(`users/${shortname}/types`),
  getEntityTypeByUserAndTitle: ({
    title,
    shortname,
  }: {
    title: string;
    shortname: string;
  }) =>
    apiClient.get<ApiTypeByUserAndTitleResponse>(
      `users/${shortname}/types/${title}`,
    ),
  createEntityType: (requestData: ApiTypeCreateRequest) =>
    apiClient.post<ApiTypeCreateRequest, ApiTypeCreateResponse>(
      "types/create",
      requestData,
    ),
  updateEntityType: (requestData: ApiTypeUpdateRequest, entityTypeId: string) =>
    apiClient.put<ApiTypeUpdateRequest, ApiTypeUpdateResponse>(
      `types/${entityTypeId}/update`,
      requestData,
    ),
  signup: (requestData: ApiSignupRequestBody) =>
    post<ApiSignupRequestBody, ApiSignupResponse>("signup", requestData),
  verifyEmail: (requestData: ApiVerifyEmailRequestBody) =>
    apiClient.post<ApiVerifyEmailRequestBody, ApiVerifyEmailResponse>(
      "verify-email",
      requestData,
    ),
  sendLoginCode: (requestData: ApiSendLoginCodeRequestBody) =>
    apiClient.post<ApiSendLoginCodeRequestBody, ApiSendLoginCodeResponse>(
      "send-login-code",
      requestData,
    ),
  loginWithLoginCode: (requestData: ApiLoginWithLoginCodeRequestBody) =>
    apiClient.post<
      ApiLoginWithLoginCodeRequestBody,
      ApiLoginWithLoginCodeResponse
    >("login-with-login-code", requestData),
};
