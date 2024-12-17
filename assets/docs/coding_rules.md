# Project Documentation: Auto Generate Flutter Project

## Coding Roles and Prefix Guidelines

Berikut adalah pedoman penamaan dan struktur kode untuk project Flutter dengan arsitektur Clean dan modular:

---

### 1. **Model Level Data (DTO)**

- Gunakan prefix `dto` untuk model yang digunakan sebagai representasi **data** dari response API atau sumber data lainnya.
- **Contoh**:
   ```dart
   class UserDto {
       final String id;
       final String name;

       UserDto({required this.id, required this.name});

       factory UserDto.fromJson(Map<String, dynamic> json) {
           return UserDto(
               id: json['id'],
               name: json['name'],
           );
       }
   }
   ```

---

### 2. **Model Level Domain**

- Gunakan prefix `model` untuk model yang akan digunakan di level **domain** (business logic).
- **Contoh**:
   ```dart
   class UserModel {
       final String id;
       final String name;

       UserModel({required this.id, required this.name});
   }
   ```

---

### 3. **Use Case**

- Gunakan nama dengan format `<verb>_<subject>_use_case`.
- Use Case mengatur business logic spesifik dengan dua **class**:
  - **Input**: Data yang dibutuhkan untuk memproses Use Case.
  - **Output**: Data hasil proses Use Case.
- **Contoh**:
   ```dart
   // Input Class
   class GetUserInput {
       final String userId;
       GetUserInput(this.userId);
   }

   // Output Class
   class GetUserOutput {
       final UserModel user;
       GetUserOutput(this.user);
   }

   // Use Case
   class GetUserUseCase {
       Future<GetUserOutput> execute(GetUserInput input) async {
           // Contoh fetching dari Repository
           return userRepository.getUserById(input.userId);
       }
   }
   ```

---

### 4. **Repository**

- Prefix nama repository dengan `repository` atau tambahkan postfix `Repository`.
- **Contoh**:
   ```dart
   abstract class UserRepository {
       Future<GetUserByIdOutput> getUserById(String id);
   }
   ```

---

## Best Practices for Flutter Project

### 1. **Folder Structure**

Gunakan struktur folder Clean Architecture yang modular:

```
packages/
  ├── library/
  │   ├── core/
  │   └── resource/
  ├── data/
  │   ├── data_common/
  │   ├── data_auth/
  │   └── data_analytic/
  │       ├── di/
  │       ├── config/
  │       ├── model/
  │       ├── mapper/
  │       └── repository/
  ├── domain/
  │   ├── domain_common/
  │   ├── domain_auth/
  │   └── domain_analytic/
  │       ├── di/
  │       ├── config/
  │       ├── model/
  │       ├── use_case/
  │       └── repository/
  └── presentation/
      ├── feature_common/
      ├── feature_dashboard/
      └── feature_auth/
          ├── bloc/
          ├── config/
          ├── di/
          └── page/
```

### 2. **Use State Management**

- Gunakan **BLoC** atau **Cubit** untuk state management. Hindari mencampurkan logic langsung di dalam UI.

### 3. **Naming Conventions**

- Gunakan camelCase untuk variabel dan method.
- Gunakan PascalCase untuk class.
- Gunakan Snak_Case untuk file.
- Contoh:
  - `UserRepository`
  - `getUserById`
  - `userInput`
  - `login_page.dart`

### 4. **Avoid Hardcoding**

- Simpan konfigurasi seperti URL, keys, atau teks dalam file terpisah, seperti `constants.dart` atau `env`.

### 5. **Dependency Injection**

- Gunakan package seperti `get_it` dan `injectable` untuk **dependency injection**. Ini akan membuat kode lebih modular dan mudah di-maintain.

### 6. **Error Handling**

- Selalu gunakan `try-catch` pada request ke API.
- Pastikan setiap error ditangani dengan output user-friendly.

### 7. **Class Mapper (Avoid Extension Functions)**

- Gunakan **class mapper** untuk mengonversi data antar layer, seperti dari `DTO` ke `Model`.
- Jangan gunakan **extension functions** untuk mapping karena extension tidak dapat di-inject dan lebih sulit di-maintain dalam proyek besar.
- Dengan menggunakan class mapper, kita bisa memanfaatkan **dependency injection** dan membuat kode lebih modular dan testable.

**Alasan Menghindari Extension Function**:

1. Tidak dapat di-inject menggunakan **dependency injection**.
2. Tidak modular, sulit diatur saat proyek semakin besar.
3. Mapper dalam bentuk class lebih mudah diuji (unit testing).

### Repository with Mapper Integration

- **Gunakan mapper di repository** agar data yang diambil dari sumber data dapat langsung dikonversi ke model level domain.

#### Contoh Implementasi:

```dart
// GetUserByIdOutput Class
class GetUserByIdOutput {
    GetUserByIdOutput mapFromDto(UserDto dto) {
        return GetUserByIdOutput();
    }
}

// Repository Implementation
class UserRepositoryImpl implements UserRepository {
    final UserDataSource _dataSource;
    final GetUserByIdOutput _mapper;

    UserRepositoryImpl(this._dataSource, this._mapper);

    @override
    Future<GetUserByIdOutput> getUserById(String id) async {
        final response = await _dataSource.fetchUser(id);
        return _mapper.mapToModel(response);
    }
}
```

#### Use Case Example with Repository

```dart
class GetUserByIdUseCase {
    final UserRepositoryImpl _repository;

    GetUserUseCase(this._repository);

    Future<GetUserByIdOutput> call(GetUserInput input) async {
        return _repository.getUserModelById(input.userId);
    }
}
```

### Keuntungan Menggunakan Mapper di Repository:

1. Repository bertanggung jawab atas konversi data dari sumber data ke model domain.
2. Use case tetap bersih dan hanya berfokus pada business logic.
3. Mapper terisolasi dan dapat digunakan kembali di berbagai tempat.

#### Penjelasan:

1. **Repository** menangani seluruh proses:

- Fetching data DTO.
- Mapping dari DTO ke Domain Model menggunakan `UserMapper`.
- Return **Output** langsung ke Use Case.

2. **Use Case** hanya fokus pada:

- Menerima input.
- Mengambil output langsung dari repository.

3. **Hasil**: Use Case menjadi lebih bersih, fokus, dan lebih mudah untuk di-maintain.

---

## Notes

1. Pisahkan business logic dari UI dan network layers.
2. Selalu buat unit test untuk Use Case, Model, dan Repository.
3. Gunakan linting rules yang ketat (`package:lint`) untuk memastikan kode konsisten.
