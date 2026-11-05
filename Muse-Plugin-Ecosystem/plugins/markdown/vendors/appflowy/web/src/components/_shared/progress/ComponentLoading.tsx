import CircularProgress from '@mui/material/CircularProgress';

function ComponentLoading() {
  return (
    <div className={'flex h-[260px] w-full items-center justify-center'}>
      <CircularProgress />
    </div>
  );
}

export default ComponentLoading;
