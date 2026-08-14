/// Application failure types for error handling.
/// Failures represent expected errors that can occur during app operation.
abstract class Failure {
  final String message;
  const Failure(this.message);

  @override
  String toString() => message;
}

/// Failure when engine initialization fails.
class EngineInitFailure extends Failure {
  const EngineInitFailure(super.message);
}

/// Failure when a factory operation fails.
class FactoryFailure extends Failure {
  final String? factoryId;
  const FactoryFailure(super.message, {this.factoryId});
}

/// Failure when a logistics operation fails.
class LogisticsFailure extends Failure {
  final String? logisticsId;
  const LogisticsFailure(super.message, {this.logisticsId});
}

/// Failure when a blueprint operation fails.
class BlueprintFailure extends Failure {
  final String? blueprintId;
  const BlueprintFailure(super.message, {this.blueprintId});
}

/// Failure when save/load operation fails.
class StorageFailure extends Failure {
  const StorageFailure(super.message);
}

/// Failure when engine operation fails.
class EngineFailure extends Failure {
  const EngineFailure(super.message);
}

/// Generic unknown failure.
class UnknownFailure extends Failure {
  const UnknownFailure([super.message = 'An unknown error occurred']);
}
