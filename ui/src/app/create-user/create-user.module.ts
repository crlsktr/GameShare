import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { NewUserFormComponent } from './new-user-form/new-user-form.component';
import { ReactiveFormsModule } from '@angular/forms';



@NgModule({
	declarations: [NewUserFormComponent],
	exports: [NewUserFormComponent],
	imports: [
		CommonModule,
		ReactiveFormsModule
	]
})
export class CreateUserModule { }
