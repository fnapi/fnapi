import { Context, FnApi, Http, Validate, validators } from "@fnapi/api";

export default class AuthApi {
  /**
   * 'required' is implicitly added, if argument is not optional.
   *
   * @param email
   * @param password
   */
  @FnApi()
  @Http({
    method: "POST",
    path: "/logi",
  })
  @Validate("email", [validators.email])
  @Validate("password", [validators.required, validators.minLenth(8)])
  static async login(email: string, password: string): Promise<string> {
    const user = Context.get<User>();
  }
}
