const TableSkeleton = ({ rows = 5, columns = 4 }: { rows?: number; columns?: number }) => {
  return (
    <div className='mt-4 overflow-x-auto shadow-md sm:rounded-lg'>
      <table className='w-full text-left text-sm text-gray-500'>
        <thead className='bg-fill-content-hover text-xs uppercase text-gray-700 dark:text-gray-400'>
          <tr>
            {[...Array(columns)].map((_, index) => (
              <th key={`header-${index}`} scope='col' className='px-6 py-3'>
                <div className='h-6 w-24 animate-pulse rounded bg-fill-content-hover'></div>
              </th>
            ))}
          </tr>
        </thead>
        <tbody>
          {[...Array(rows)].map((_, rowIndex) => (
            <tr key={`row-${rowIndex}`} className='border-b border-border-primary bg-background-primary'>
              {[...Array(columns)].map((_, colIndex) => (
                <td key={`cell-${rowIndex}-${colIndex}`} className='px-6 py-4'>
                  <div
                    className={`h-5 rounded bg-fill-content-hover ${colIndex === 0 ? 'w-14' : 'w-24'} animate-pulse`}
                  ></div>
                </td>
              ))}
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
};

export default TableSkeleton;
