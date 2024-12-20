import 'dart:io';

final class Shared {
  const Shared._();

  static Future<List<T>> readFile<T>(String fileName, {
    required T Function(String) lineTransformer,
  }) async {
    final File file = File.fromUri(Uri.file(fileName));

    final List<String> lines = await file.readAsLines();

    return lines.map(lineTransformer).toList();
  }
}
