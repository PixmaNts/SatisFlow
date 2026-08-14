import 'package:go_router/go_router.dart';

import '../../features/dashboard/dashboard_page.dart';
import '../../features/factory/factory_page.dart';
import '../../features/logistics/logistics_page.dart';
import '../../features/blueprints/blueprints_page.dart';
import 'scaffold_with_nav.dart';

/// Application router configuration using go_router.
class AppRouter {
  AppRouter._();

  static final GoRouter router = GoRouter(
    initialLocation: '/',
    routes: [
      StatefulShellRoute.indexedStack(
        builder: (context, state, navigationShell) {
          return ScaffoldWithNav(navigationShell: navigationShell);
        },
        branches: [
          StatefulShellBranch(
            routes: [
              GoRoute(
                path: '/',
                name: 'dashboard',
                pageBuilder: (context, state) =>
                    const NoTransitionPage(child: DashboardPage()),
              ),
            ],
          ),
          StatefulShellBranch(
            routes: [
              GoRoute(
                path: '/factory',
                name: 'factory',
                pageBuilder: (context, state) =>
                    const NoTransitionPage(child: FactoryPage()),
              ),
            ],
          ),
          StatefulShellBranch(
            routes: [
              GoRoute(
                path: '/logistics',
                name: 'logistics',
                pageBuilder: (context, state) =>
                    const NoTransitionPage(child: LogisticsPage()),
              ),
            ],
          ),
          StatefulShellBranch(
            routes: [
              GoRoute(
                path: '/blueprints',
                name: 'blueprints',
                pageBuilder: (context, state) =>
                    const NoTransitionPage(child: BlueprintsPage()),
              ),
            ],
          ),
        ],
      ),
    ],
  );
}
