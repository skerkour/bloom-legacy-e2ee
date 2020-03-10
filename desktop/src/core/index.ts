import { ipcRenderer } from 'electron';
import * as models from '@/api/models';
import { log } from '@/libs/rz';

const Empty = {};

async function call(method: string, params: any): Promise<any> {
  const message = JSON.stringify({
    method,
    params,
  });
  log.with({ msg: message }).debug('jsonMessage');

  const res: any = await ipcRenderer.invoke('core:call', message);
  log.with({ res: res.data }).debug('resMessage');

  const { data } = res;
  if (data.error !== null) {
    throw data.error;
  }

  return data.data;
}

function toIsoDate(date: string | null): Date | null {
  if (date === null) {
    return null;
  }
  return new Date(date).toISOString() as unknown as Date;
}

async function init(): Promise<models.SignedIn | null> {
  return call('core.init', Empty);
}

export default {
  call,
  toIsoDate,
  Empty,
  init,
};