import { Routes } from '@angular/router';

export const routes: Routes = [
  {
    path: 'map',
    loadComponent: () => import('./features/map/pages/map-page/map-page.page').then(m => m.MapPagePage)
  },
  {
    path: '',
    redirectTo: 'map',
    pathMatch: 'full'
  }
];
