export interface UserType {
  id: number;
  loginname: string;
}
export interface RustResponse<T> {
  status: 'success' | 'failed';
  data: T | string;
}
