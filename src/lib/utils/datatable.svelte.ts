export class DataTableState<T> {
  data = $state<T[]>([]);
  search = $state('');
  currentPage = $state(1);
  pageSize = $state(20);
  sortColumn = $state<string | null>(null);
  sortDirection = $state<'asc' | 'desc'>('asc');
  searchFields: string[];

  constructor(initialData: T[], searchFields: string[]) {
    this.data = initialData;
    this.searchFields = searchFields;
  }

  private getNestedValue(obj: any, path: string): any {
    if (!obj) return undefined;
    return path.split('.').reduce((acc, part) => {
      if (acc === null || acc === undefined) return undefined;
      return acc[part];
    }, obj);
  }

  get filtered() {
    if (!this.search.trim()) return this.data;
    const query = this.search.toLowerCase().trim();
    return this.data.filter(item =>
      this.searchFields.some(field => {
        const val = this.getNestedValue(item, field);
        return val != null && String(val).toLowerCase().includes(query);
      })
    );
  }

  get sorted() {
    const list = [...this.filtered];
    if (!this.sortColumn) return list;

    const col = this.sortColumn;
    const dir = this.sortDirection === 'asc' ? 1 : -1;

    list.sort((a, b) => {
      const valA = this.getNestedValue(a, col);
      const valB = this.getNestedValue(b, col);

      if (valA === valB) return 0;
      if (valA == null) return 1;
      if (valB == null) return -1;

      // Handle simple numeric sorting
      const numA = Number(valA);
      const numB = Number(valB);
      if (!isNaN(numA) && !isNaN(numB) && typeof valA !== 'boolean' && typeof valB !== 'boolean') {
        return (numA - numB) * dir;
      }

      // Handle string locale sorting (case insensitive)
      return String(valA).localeCompare(String(valB), 'es', { numeric: true, sensitivity: 'base' }) * dir;
    });
    return list;
  }

  get paginated() {
    const start = (this.currentPage - 1) * this.pageSize;
    const end = start + this.pageSize;
    return this.sorted.slice(start, end);
  }

  get totalPages() {
    return Math.max(1, Math.ceil(this.sorted.length / this.pageSize));
  }

  get startIndex() {
    if (this.sorted.length === 0) return 0;
    return (this.currentPage - 1) * this.pageSize + 1;
  }

  get endIndex() {
    return Math.min(this.currentPage * this.pageSize, this.sorted.length);
  }

  sortBy(column: string) {
    if (this.sortColumn === column) {
      this.sortDirection = this.sortDirection === 'asc' ? 'desc' : 'asc';
    } else {
      this.sortColumn = column;
      this.sortDirection = 'asc';
    }
    this.currentPage = 1;
  }
}
