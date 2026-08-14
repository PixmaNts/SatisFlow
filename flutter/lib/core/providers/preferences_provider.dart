import 'dart:convert';

import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:shared_preferences/shared_preferences.dart';

/// Storage key for preferences
const _preferencesKey = 'satisflow-preferences';

/// User preferences data model
class UserPreferences {
  final String? selectedFactoryId;
  final DashboardFilters dashboardFilters;
  final String factoryViewTab;
  final LogisticsFilters logisticsFilters;

  const UserPreferences({
    this.selectedFactoryId,
    this.dashboardFilters = const DashboardFilters(),
    this.factoryViewTab = 'production',
    this.logisticsFilters = const LogisticsFilters(),
  });

  UserPreferences copyWith({
    String? selectedFactoryId,
    DashboardFilters? dashboardFilters,
    String? factoryViewTab,
    LogisticsFilters? logisticsFilters,
  }) {
    return UserPreferences(
      selectedFactoryId: selectedFactoryId ?? this.selectedFactoryId,
      dashboardFilters: dashboardFilters ?? this.dashboardFilters,
      factoryViewTab: factoryViewTab ?? this.factoryViewTab,
      logisticsFilters: logisticsFilters ?? this.logisticsFilters,
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'selectedFactoryId': selectedFactoryId,
      'dashboardFilters': dashboardFilters.toJson(),
      'factoryViewTab': factoryViewTab,
      'logisticsFilters': logisticsFilters.toJson(),
    };
  }

  factory UserPreferences.fromJson(Map<String, dynamic> json) {
    return UserPreferences(
      selectedFactoryId: json['selectedFactoryId'] as String?,
      dashboardFilters: json['dashboardFilters'] != null
          ? DashboardFilters.fromJson(
              json['dashboardFilters'] as Map<String, dynamic>,
            )
          : const DashboardFilters(),
      factoryViewTab: json['factoryViewTab'] as String? ?? 'production',
      logisticsFilters: json['logisticsFilters'] != null
          ? LogisticsFilters.fromJson(
              json['logisticsFilters'] as Map<String, dynamic>,
            )
          : const LogisticsFilters(),
    );
  }

  static const defaults = UserPreferences();
}

/// Dashboard filter state
class DashboardFilters {
  final String state;
  final String searchText;
  final String sortBy;
  final String sortOrder;

  const DashboardFilters({
    this.state = 'all',
    this.searchText = '',
    this.sortBy = 'name',
    this.sortOrder = 'asc',
  });

  DashboardFilters copyWith({
    String? state,
    String? searchText,
    String? sortBy,
    String? sortOrder,
  }) {
    return DashboardFilters(
      state: state ?? this.state,
      searchText: searchText ?? this.searchText,
      sortBy: sortBy ?? this.sortBy,
      sortOrder: sortOrder ?? this.sortOrder,
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'state': state,
      'searchText': searchText,
      'sortBy': sortBy,
      'sortOrder': sortOrder,
    };
  }

  factory DashboardFilters.fromJson(Map<String, dynamic> json) {
    return DashboardFilters(
      state: json['state'] as String? ?? 'all',
      searchText: json['searchText'] as String? ?? '',
      sortBy: json['sortBy'] as String? ?? 'name',
      sortOrder: json['sortOrder'] as String? ?? 'asc',
    );
  }
}

/// Logistics filter state
class LogisticsFilters {
  final String transportType;
  final String sourceFactory;
  final String destinationFactory;
  final String searchText;

  const LogisticsFilters({
    this.transportType = 'all',
    this.sourceFactory = 'all',
    this.destinationFactory = 'all',
    this.searchText = '',
  });

  LogisticsFilters copyWith({
    String? transportType,
    String? sourceFactory,
    String? destinationFactory,
    String? searchText,
  }) {
    return LogisticsFilters(
      transportType: transportType ?? this.transportType,
      sourceFactory: sourceFactory ?? this.sourceFactory,
      destinationFactory: destinationFactory ?? this.destinationFactory,
      searchText: searchText ?? this.searchText,
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'transportType': transportType,
      'sourceFactory': sourceFactory,
      'destinationFactory': destinationFactory,
      'searchText': searchText,
    };
  }

  factory LogisticsFilters.fromJson(Map<String, dynamic> json) {
    return LogisticsFilters(
      transportType: json['transportType'] as String? ?? 'all',
      sourceFactory: json['sourceFactory'] as String? ?? 'all',
      destinationFactory: json['destinationFactory'] as String? ?? 'all',
      searchText: json['searchText'] as String? ?? '',
    );
  }
}

/// AsyncNotifier for managing preferences with shared_preferences persistence
class PreferencesNotifier extends AsyncNotifier<UserPreferences> {
  late SharedPreferences _prefs;

  @override
  Future<UserPreferences> build() async {
    _prefs = await SharedPreferences.getInstance();
    return _loadPreferences();
  }

  UserPreferences _loadPreferences() {
    try {
      final stored = _prefs.getString(_preferencesKey);
      if (stored != null) {
        final json = jsonDecode(stored) as Map<String, dynamic>;
        return UserPreferences.fromJson(json);
      }
    } catch (e) {
      // Reset to defaults on error
    }
    return UserPreferences.defaults;
  }

  Future<void> _savePreferences(UserPreferences prefs) async {
    try {
      final json = jsonEncode(prefs.toJson());
      await _prefs.setString(_preferencesKey, json);
    } catch (e) {
      // Log error but don't throw - preferences are non-critical
    }
  }

  /// Update selected factory ID
  Future<void> setSelectedFactoryId(String? factoryId) async {
    final updated = state.requireValue.copyWith(selectedFactoryId: factoryId);
    state = AsyncValue.data(updated);
    await _savePreferences(updated);
  }

  /// Update dashboard filters partially
  Future<void> updateDashboardFilters({
    String? filterState,
    String? searchText,
    String? sortBy,
    String? sortOrder,
  }) async {
    final current = state.requireValue;
    final updated = current.copyWith(
      dashboardFilters: current.dashboardFilters.copyWith(
        state: filterState,
        searchText: searchText,
        sortBy: sortBy,
        sortOrder: sortOrder,
      ),
    );
    state = AsyncValue.data(updated);
    await _savePreferences(updated);
  }

  /// Reset dashboard filters to defaults
  Future<void> resetDashboardFilters() async {
    final updated = state.requireValue.copyWith(
      dashboardFilters: const DashboardFilters(),
    );
    state = AsyncValue.data(updated);
    await _savePreferences(updated);
  }

  /// Update factory view tab
  Future<void> setFactoryViewTab(String tab) async {
    final updated = state.requireValue.copyWith(factoryViewTab: tab);
    state = AsyncValue.data(updated);
    await _savePreferences(updated);
  }

  /// Update logistics filters partially
  Future<void> updateLogisticsFilters({
    String? transportType,
    String? sourceFactory,
    String? destinationFactory,
    String? searchText,
  }) async {
    final updated = state.requireValue.copyWith(
      logisticsFilters: state.requireValue.logisticsFilters.copyWith(
        transportType: transportType,
        sourceFactory: sourceFactory,
        destinationFactory: destinationFactory,
        searchText: searchText,
      ),
    );
    state = AsyncValue.data(updated);
    await _savePreferences(updated);
  }

  /// Reset logistics filters to defaults
  Future<void> resetLogisticsFilters() async {
    final updated = state.requireValue.copyWith(
      logisticsFilters: const LogisticsFilters(),
    );
    state = AsyncValue.data(updated);
    await _savePreferences(updated);
  }

  /// Reset all preferences to defaults
  Future<void> resetPreferences() async {
    final updated = UserPreferences.defaults;
    state = AsyncValue.data(updated);
    await _savePreferences(updated);
  }
}

/// Main preferences provider
final preferencesProvider =
    AsyncNotifierProvider<PreferencesNotifier, UserPreferences>(
      PreferencesNotifier.new,
    );

/// Convenience provider for selected factory ID
final selectedFactoryIdProvider = Provider<String?>((ref) {
  return ref
      .watch(preferencesProvider)
      .maybeWhen(data: (prefs) => prefs.selectedFactoryId, orElse: () => null);
});

/// Convenience provider for dashboard filters
final dashboardFiltersProvider = Provider<DashboardFilters>((ref) {
  return ref
      .watch(preferencesProvider)
      .maybeWhen(
        data: (prefs) => prefs.dashboardFilters,
        orElse: () => const DashboardFilters(),
      );
});

/// Convenience provider for factory view tab
final factoryViewTabProvider = Provider<String>((ref) {
  return ref
      .watch(preferencesProvider)
      .maybeWhen(
        data: (prefs) => prefs.factoryViewTab,
        orElse: () => 'production',
      );
});

/// Convenience provider for logistics filters
final logisticsFiltersProvider = Provider<LogisticsFilters>((ref) {
  return ref
      .watch(preferencesProvider)
      .maybeWhen(
        data: (prefs) => prefs.logisticsFilters,
        orElse: () => const LogisticsFilters(),
      );
});
