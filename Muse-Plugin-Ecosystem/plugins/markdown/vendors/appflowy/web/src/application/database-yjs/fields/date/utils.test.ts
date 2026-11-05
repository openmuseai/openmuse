import { DateFormat, TimeFormat } from '@/application/types';
import { getDateFormat, getTimeFormat } from '@/utils/time';
import { expect } from '@jest/globals';

describe('DateFormat', () => {
  it('should return time format', () => {
    expect(getTimeFormat(TimeFormat.TwelveHour)).toEqual('h:mm A');
    expect(getTimeFormat(TimeFormat.TwentyFourHour)).toEqual('HH:mm');
    expect(getTimeFormat(56)).toEqual('HH:mm');
  });

  it('should return date format', () => {
    expect(getDateFormat(DateFormat.US)).toEqual('YYYY/MM/DD');
    expect(getDateFormat(DateFormat.ISO)).toEqual('YYYY-MM-DD');
    expect(getDateFormat(DateFormat.Friendly)).toEqual('MMM DD, YYYY');
    expect(getDateFormat(DateFormat.Local)).toEqual('MM/DD/YYYY');
    expect(getDateFormat(DateFormat.DayMonthYear)).toEqual('DD/MM/YYYY');

    expect(getDateFormat(56)).toEqual('YYYY-MM-DD');
  });
});
