import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { NewUserFormComponent } from './new-user-form/new-user-form.component';



@NgModule({
  declarations: [NewUserFormComponent],
  exports: [NewUserFormComponent],
  imports: [
    CommonModule
  ]
})
export class CreateUserModule { }
