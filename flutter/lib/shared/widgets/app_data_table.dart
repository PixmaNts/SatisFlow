import 'package:flutter/material.dart';

/// A sortable, filterable, paginated data table widget.
///
/// Uses Theme.of(context) for all colors - no hardcoded values.
class AppDataTable<T> extends StatefulWidget {
  /// The list of columns to display.
  final List<DataTableColumn<T>> columns;

  /// The list of data items.
  final List<T> data;

  /// Callback when a row is tapped.
  final ValueChanged<T>? onRowTap;

  /// Callback when a row is long-pressed.
  final ValueChanged<T>? onRowLongPress;

  /// Initial sort column index.
  final int? initialSortColumnIndex;

  /// Initial sort ascending.
  final bool initialSortAscending;

  /// Number of rows per page.
  final int rowsPerPage;

  /// Available rows per page options.
  final List<int> rowsPerPageOptions;

  /// Whether to show pagination.
  final bool showPagination;

  /// Whether to show the rows per page selector.
  final bool showRowsPerPage;

  /// Filter function for filtering data.
  final List<T> Function(List<T> data, String query)? filter;

  /// Whether to show the filter field.
  final bool showFilter;

  /// Filter hint text.
  final String filterHint;

  /// Empty state widget when no data.
  final Widget? emptyState;

  /// Loading indicator.
  final bool isLoading;

  /// Creates an AppDataTable.
  const AppDataTable({
    super.key,
    required this.columns,
    required this.data,
    this.onRowTap,
    this.onRowLongPress,
    this.initialSortColumnIndex,
    this.initialSortAscending = true,
    this.rowsPerPage = 10,
    this.rowsPerPageOptions = const [5, 10, 25, 50],
    this.showPagination = true,
    this.showRowsPerPage = true,
    this.filter,
    this.showFilter = true,
    this.filterHint = 'Filter...',
    this.emptyState,
    this.isLoading = false,
  });

  @override
  State<AppDataTable<T>> createState() => _AppDataTableState<T>();
}

class _AppDataTableState<T> extends State<AppDataTable<T>> {
  int _sortColumnIndex = 0;
  bool _sortAscending = true;
  int _currentPage = 0;
  int _rowsPerPage = 10;
  String _filterQuery = '';
  late List<T> _filteredData;
  late List<T> _sortedData;

  @override
  void initState() {
    super.initState();
    _sortColumnIndex = widget.initialSortColumnIndex ?? 0;
    _sortAscending = widget.initialSortAscending;
    _rowsPerPage = widget.rowsPerPage;
    _filteredData = widget.data;
    _sortedData = List.from(_filteredData);
    _sortData();
  }

  @override
  void didUpdateWidget(AppDataTable<T> oldWidget) {
    super.didUpdateWidget(oldWidget);
    if (widget.data != oldWidget.data) {
      _applyFilter();
    }
  }

  void _applyFilter() {
    setState(() {
      if (widget.filter != null && _filterQuery.isNotEmpty) {
        _filteredData = widget.filter!(widget.data, _filterQuery);
      } else {
        _filteredData = List.from(widget.data);
      }
      _sortedData = List.from(_filteredData);
      _sortData();
      _currentPage = 0;
    });
  }

  void _sortData() {
    if (_sortColumnIndex >= widget.columns.length) return;

    final column = widget.columns[_sortColumnIndex];
    if (column.sorter == null) return;

    _sortedData.sort((a, b) {
      final comparison = column.sorter!(a, b);
      return _sortAscending ? comparison : -comparison;
    });
  }

  void _handleSort(int columnIndex) {
    setState(() {
      if (_sortColumnIndex == columnIndex) {
        _sortAscending = !_sortAscending;
      } else {
        _sortColumnIndex = columnIndex;
        _sortAscending = true;
      }
      _sortData();
    });
  }

  void _handlePageChanged(int page) {
    setState(() {
      _currentPage = page;
    });
  }

  void _handleRowsPerPageChanged(int? value) {
    if (value != null) {
      setState(() {
        _rowsPerPage = value;
        _currentPage = 0;
      });
    }
  }

  void _handleFilterChanged(String query) {
    setState(() {
      _filterQuery = query;
    });
    _applyFilter();
  }

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);

    if (widget.isLoading) {
      return const Center(child: CircularProgressIndicator());
    }

    if (_sortedData.isEmpty && _filterQuery.isNotEmpty) {
      return widget.emptyState ??
          Center(
            child: Text(
              'No results found for "$_filterQuery"',
              style: theme.textTheme.bodyLarge,
            ),
          );
    }

    if (_sortedData.isEmpty) {
      return widget.emptyState ??
          Center(
            child: Text('No data available', style: theme.textTheme.bodyLarge),
          );
    }

    final startIndex = _currentPage * _rowsPerPage;
    final endIndex = (startIndex + _rowsPerPage).clamp(0, _sortedData.length);
    final pageData = _sortedData.sublist(startIndex, endIndex);
    final totalPages = (_sortedData.length / _rowsPerPage).ceil();

    final table = SingleChildScrollView(
      child: SizedBox(
        width: double.infinity,
        child: DataTable(
          sortColumnIndex: _sortColumnIndex < widget.columns.length
              ? _sortColumnIndex
              : null,
          sortAscending: _sortAscending,
          columns: widget.columns.map((column) {
            return DataColumn(
              label: Expanded(
                child: Text(
                  column.label,
                  style: theme.textTheme.titleSmall?.copyWith(
                    fontWeight: FontWeight.bold,
                  ),
                ),
              ),
              numeric: column.numeric,
              onSort: column.sorter != null
                  ? (columnIndex, ascending) =>
                        _handleSort(widget.columns.indexOf(column))
                  : null,
            );
          }).toList(),
          rows: pageData.map((item) {
            return DataRow(
              onSelectChanged: widget.onRowTap != null
                  ? (_) => widget.onRowTap!(item)
                  : null,
              onLongPress: widget.onRowLongPress != null
                  ? () => widget.onRowLongPress!(item)
                  : null,
              cells: widget.columns.map((column) {
                return DataCell(column.cellBuilder(context, item));
              }).toList(),
            );
          }).toList(),
        ),
      ),
    );

    return LayoutBuilder(
      builder: (context, constraints) {
        final hasBoundedHeight = constraints.maxHeight.isFinite;
        final tableSection = hasBoundedHeight ? Expanded(child: table) : table;

        return Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          mainAxisSize: hasBoundedHeight ? MainAxisSize.max : MainAxisSize.min,
          children: [
            // Filter field
            if (widget.showFilter)
              Padding(
                padding: const EdgeInsets.only(bottom: 16),
                child: TextField(
                  decoration: InputDecoration(
                    hintText: widget.filterHint,
                    prefixIcon: const Icon(Icons.search),
                    suffixIcon: _filterQuery.isNotEmpty
                        ? IconButton(
                            icon: const Icon(Icons.clear),
                            onPressed: () => _handleFilterChanged(''),
                          )
                        : null,
                    border: OutlineInputBorder(
                      borderRadius: BorderRadius.circular(8),
                    ),
                    contentPadding: const EdgeInsets.symmetric(
                      horizontal: 12,
                      vertical: 8,
                    ),
                  ),
                  onChanged: _handleFilterChanged,
                ),
              ),

            // Table
            tableSection,

            // Pagination
            if (widget.showPagination && totalPages > 1)
              Padding(
                padding: const EdgeInsets.only(top: 16),
                child: Row(
                  mainAxisAlignment: MainAxisAlignment.spaceBetween,
                  children: [
                    // Rows per page
                    if (widget.showRowsPerPage)
                      Row(
                        children: [
                          Text(
                            'Rows per page:',
                            style: theme.textTheme.bodySmall,
                          ),
                          const SizedBox(width: 8),
                          DropdownButton<int>(
                            value: _rowsPerPage,
                            onChanged: _handleRowsPerPageChanged,
                            items: widget.rowsPerPageOptions.map((value) {
                              return DropdownMenuItem(
                                value: value,
                                child: Text('$value'),
                              );
                            }).toList(),
                            underline: Container(),
                            style: theme.textTheme.bodySmall,
                          ),
                        ],
                      ),

                    // Page info
                    Text(
                      '${startIndex + 1}-$endIndex of ${_sortedData.length}',
                      style: theme.textTheme.bodySmall,
                    ),

                    // Page navigation
                    Row(
                      children: [
                        IconButton(
                          icon: const Icon(Icons.chevron_left),
                          onPressed: _currentPage > 0
                              ? () => _handlePageChanged(_currentPage - 1)
                              : null,
                          tooltip: 'Previous page',
                        ),
                        Text(
                          '${_currentPage + 1} / $totalPages',
                          style: theme.textTheme.bodySmall,
                        ),
                        IconButton(
                          icon: const Icon(Icons.chevron_right),
                          onPressed: _currentPage < totalPages - 1
                              ? () => _handlePageChanged(_currentPage + 1)
                              : null,
                          tooltip: 'Next page',
                        ),
                      ],
                    ),
                  ],
                ),
              ),
          ],
        );
      },
    );
  }
}

/// A column definition for the data table.
class DataTableColumn<T> {
  /// The column label.
  final String label;

  /// Whether the column contains numeric data.
  final bool numeric;

  /// Builder function for the cell content.
  final Widget Function(BuildContext context, T item) cellBuilder;

  /// Optional sorter function for sorting.
  final int Function(T a, T b)? sorter;

  /// Creates a DataTableColumn.
  const DataTableColumn({
    required this.label,
    required this.cellBuilder,
    this.numeric = false,
    this.sorter,
  });

  /// Creates a text column.
  static DataTableColumn<T> text<T>({
    required String label,
    required String Function(T item) value,
    bool numeric = false,
    int Function(T a, T b)? sorter,
  }) {
    return DataTableColumn<T>(
      label: label,
      numeric: numeric,
      sorter:
          sorter ?? (numeric ? null : (a, b) => value(a).compareTo(value(b))),
      cellBuilder: (context, item) => Text(value(item)),
    );
  }

  /// Creates a numeric column.
  static DataTableColumn<T> number<T>({
    required String label,
    required num Function(T item) value,
    String Function(num value)? formatter,
  }) {
    return DataTableColumn<T>(
      label: label,
      numeric: true,
      sorter: (a, b) => value(a).compareTo(value(b)),
      cellBuilder: (context, item) {
        final numValue = value(item);
        return Text(
          formatter != null ? formatter(numValue) : numValue.toString(),
          textAlign: TextAlign.right,
        );
      },
    );
  }
}
