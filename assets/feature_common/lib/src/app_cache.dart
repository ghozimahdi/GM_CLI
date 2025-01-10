///
/// How to use?
/// void main() {
///   final cache = AppCache();
///
///   Menyimpan data
///   cache.store('myKey', {'name': 'Alice', 'age': 30});
///
///   Mengambil data
///   if (cache.hasData('myKey')) {
///     var data = cache.retrieve('myKey');
///     print(data); // Output: {'name': 'Alice', 'age': 30}
///   }
///
///   Menghapus data
///   cache.clear('myKey');
/// }
///
library;

class AppCache<T> {
  final Map<String, T> _cache = {};

  void store(String key, T response) {
    _cache[key] = response;
  }

  T? retrieve(String key) {
    return _cache[key];
  }

  bool hasData(String key) {
    return _cache.containsKey(key);
  }

  void clear(String key) {
    _cache.remove(key);
  }

  void clearAll() {
    _cache.clear();
  }
}
