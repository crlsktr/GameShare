import { Component, OnInit } from '@angular/core';
import { FormGroup, FormControl } from '@angular/forms';

@Component({
	selector: 'app-new-user-form',
	templateUrl: './new-user-form.component.html',
	styleUrls: ['./new-user-form.component.scss']
})
export class NewUserFormComponent implements OnInit {

	newUserForm :FormGroup = new FormGroup({
		userName: new FormControl(''),
		passPhrase: new FormControl('')
	});

	constructor() { }

	ngOnInit(): void {
		this.newUserForm.controls.userName.valueChanges.subscribe( x => {
			console.log("value", x);
		});
	}

}
