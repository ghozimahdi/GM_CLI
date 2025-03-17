import 'package:data_common/data_common.dart';
import 'package:dio/dio.dart';
import 'package:injectable/injectable.dart';
import 'package:retrofit/retrofit.dart';

part 'auth_data_source.g.dart';

@lazySingleton
@RestApi()
abstract class AuthDataSource {
  @factoryMethod
  factory AuthDataSource(Dio dio, DataSourceConfig config) => _AuthDataSource(
        dio,
        baseUrl: config.baseUrl,
      );

  @POST('/v1/register')
  @FormUrlEncoded()
  Future<void> createNewAccount({
    @Field('action') required String? action,
    @Field('name') required String name,
    @Field('password') required String password,
    @Field('email') required String email,
    @Field('mobile') required String mobile,
    @Field('hasAgreedToPDPA') required bool hasAgreedToPDPA,
  });

  @POST('v1/login')
  @FormUrlEncoded()
  Future<void> login({
    @Field('action') required String? action,
    @Field('username') required String username,
    @Field('password') required String password,
  });
}
