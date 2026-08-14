/// Application error types.
abstract class AppException implements Exception {
  final String message;
  const AppException(this.message);

  @override
  String toString() => message;
}

/// Error thrown when a factory is not found.
class FactoryNotFoundException extends AppException {
  final String id;
  const FactoryNotFoundException(this.id) : super('Factory not found: $id');
}

/// Error thrown when a logistics line is not found.
class LogisticsNotFoundException extends AppException {
  final String id;
  const LogisticsNotFoundException(this.id) : super('Logistics not found: $id');
}

/// Error thrown when a blueprint is not found.
class BlueprintNotFoundException extends AppException {
  final String id;
  const BlueprintNotFoundException(this.id) : super('Blueprint not found: $id');
}
